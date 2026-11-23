#!/usr/bin/env bash
set -euo pipefail

if [[ ${1:-} == --run ]]; then
    line=$((SLURM_ARRAY_TASK_ID + 1))
    OP_INSTANCE=$(sed -n "${line}p" "$2")
    export OP_INSTANCE="${OP_INSTANCE:?Instância ausente na lista do job}"
    exec ./target/release/OrienteeringProblem
fi

cd -- "$(dirname -- "${BASH_SOURCE[0]}")"
if [[ ! -x target/release/OrienteeringProblem ]]; then
    echo 'Compile primeiro com cargo build --release --locked.' >&2
    exit 1
fi

requested_partition=${NPAD_PARTITION:-amd-3tb}
partition=$(sinfo -h -o '%P' | awk -v name="$requested_partition" '
    { sub(/\*$/, "") }
    tolower($0) == tolower(name) && !found++ { print }
')
if [[ -z $partition ]]; then
    printf 'Partição %s ausente. Confira com sinfo e ajuste NPAD_PARTITION.\n' "$requested_partition" >&2
    exit 1
fi

cpus=$(sinfo -h -N -p "$partition" -o '%c' | awk '
    /^[0-9]+$/ && (!min || $1 < min) { min = $1 }
    END { if (min) print min - 10 }
')
if [[ -z $cpus || $cpus -lt 3 ]]; then
    printf 'A partição %s precisa de pelo menos 13 CPUs por nó.\n' "$partition" >&2
    exit 1
fi
# O job tem 27 métodos independentes, mais a margem de 2 CPUs do programa.
cpus=$((cpus > 29 ? 29 : cpus))

mkdir -p results-npad
run_dir=$(mktemp -d "$PWD/results-npad/run.XXXXXXXX")
manifest="$run_dir/instances.txt"
{
    find data -maxdepth 1 -type f -name '*.txt' -printf '%f\n'
    find OPLib -maxdepth 1 -type f -name '*.oplib' -printf '%f\n'
} | LC_ALL=C sort > "$manifest"
count=$(wc -l < "$manifest")
if (( count == 0 )); then
    echo 'Nenhuma instância encontrada.' >&2
    exit 1
fi

sbatch --partition="$partition" --nodes=1 --ntasks=1 --cpus-per-task="$cpus" \
    --time="${NPAD_TIME:-24:00:00}" --job-name=orienteering \
    --chdir="$PWD" --array="0-$((count - 1))" \
    --output="$run_dir/%A_%a.csv" --error="$run_dir/%A_%a.err" \
    "$PWD/npad.sh" --run "$manifest"
printf '%s instâncias, %s CPUs por job. Arquivos em %s\n' "$count" "$cpus" "$run_dir"

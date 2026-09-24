{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
  ];

  buildInputs = with pkgs; [
    cargo
    rustc
    
    # Dependências de Áudio
    alsa-lib
    
    # Dependências de Vídeo/Janela (X11)
    xorg.libX11
    xorg.libXi
    xorg.libXcursor
    xorg.libXrandr
    
    # Wayland
    wayland
    libxkbcommon
    
    # OpenGL
    libglvnd
  ];

  # O Macroquad carrega o OpenGL dinamicamente em tempo de execução.
  # Isso garante que o binário gerado encontre os drivers de vídeo.
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
    libglvnd
    xorg.libX11
    xorg.libXi
    xorg.libXcursor
    xorg.libXrandr
    wayland
    libxkbcommon
    alsa-lib
  ]);
}
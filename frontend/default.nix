{ lib
, mkNode
, nodejs_20
}:

mkNode
{
  root = ./.;
  pnpmLock = ./pnpm-lock.yaml;
  nodejs = nodejs_20;
  build = true;
  install = false;
}
rec {
  installPhase = ''
    pnpm build
    mv dist $out
  '';
}

nix-node-package: final: prev: rec {
  egiraffe-ng-frontend = prev.callPackage ./frontend {
    mkNode = nix-node-package.lib.nix-node-package prev;
  };
}

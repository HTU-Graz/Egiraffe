{
  description = "The Nix dev shell for the Egiraffe monorepo";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.11";
    nix-node-package.url = "github:mkg20001/nix-node-package/master";
  };

  outputs = { self, nixpkgs, nix-node-package, ... }:
    let
      system = "x86_64-linux";
      supportedSystems = [ "x86_64-linux" ];
      forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems (system: f system);
    in
    {
      overlays.default = import ./overlay.nix nix-node-package;

      packages = forAllSystems (system: (import nixpkgs {
        inherit system;
        overlays = [ self.overlays.default ];
      }));

      devShells."${system}".default =
        let
          pkgs = import nixpkgs {
            inherit system;
          };
        in
        pkgs.mkShell {
          # create an environment with nodejs_20 and pnpm
          packages = with pkgs; [
            nodejs_20
            nodejs_20.pkgs.pnpm
            gcc
            openssl
            pkg-config
            postgresql_16
          ];

          shellHook = ''
            echo  ===================================
            echo ' Welcome to the Egiraffe dev shell '
            echo  ===================================
            echo
            echo "Node.js version: `node --version`"
            echo "pnpm version:    `pnpm --version`"
            echo "git version:     `git --version`"
            echo "rustc version:   `rustc --version`"
            echo "docker version:  `docker --version`"
            echo "openssl version: `openssl version`"
            $SHELL
            echo  =================================
            echo ' Closed the Egiraffe dev shell '
            echo  =================================
            exec echo -n
          '';
        };
    };
}

nix-node-package: { pkgs, lib, ... }:
{
  name = "egiraffe-test";

  nodes = {
    server = { lib, pkgs, ... }: {
      imports = [ ./module.nix ];
    };
  };

  testScript = ''
    start_all()
    server.wait_for_unit("egiraffe.service")
  '';
}

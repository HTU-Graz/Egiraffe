{ config, pkgs, lib, ... }:

{
  users.users.egiraffe = {
    isSystemUser = true;
    group = "egiraffe";
  };

  users.groups.egiraffe = { };

  services.postgresql = {
    enable = true;

    ensureUsers = [{
      name = "egiraffe";
      ensureDBOwnership = true;
    }];

    ensureDatabases = [ "egiraffe" ];
  };

  systemd.services.egiraffe = {
    wantedBy = [ "multi-user.target" ];
    after = [ "postgresql.service" "network.target" ];
    wants = [ "postgresql.service" ];
    serviceConfig = {
      User = "egiraffe";
      ExecStart = "${pkgs.egiraffe-ng}/bin/egiraffe";
    };
  };
}

self:
{ config, lib, pkgs, ... }:

let
  cfg = config.services.vocal-time-counter;
in
{
  options.services.vocal-time-counter = {
    enable = lib.mkEnableOption "the Vocal Time Counter Discord bot";

    package = lib.mkOption {
      type = lib.types.package;
      default = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
      defaultText = lib.literalExpression "vocal-time-counter.packages.\${system}.default";
      description = "Package providing the bot binary.";
    };

    environmentFile = lib.mkOption {
      type = lib.types.path;
      example = "/run/secrets/vocal-time-counter.env";
      description = ''
        File containing the bot secrets, e.g.:
          TOKEN=...
          GUILD_ID=...
        Kept out of the Nix store on purpose.
      '';
    };
  };

  config = lib.mkIf cfg.enable {
    systemd.services.vocal-time-counter = {
      description = "Vocal Time Counter Discord bot";
      wantedBy = [ "multi-user.target" ];
      wants = [ "network-online.target" ];
      after = [ "network-online.target" ];

      serviceConfig = {
        ExecStart = lib.getExe cfg.package;
        EnvironmentFile = cfg.environmentFile;
        Restart = "on-failure";
        RestartSec = 10;

        # The bot stores its data in ./data/data.json (relative to the CWD).
        # i.e. /var/lib/vocal-time-counter/data/data.json
        DynamicUser = true;
        StateDirectory = "vocal-time-counter vocal-time-counter/data";
        WorkingDirectory = "/var/lib/vocal-time-counter";

        NoNewPrivileges = true;
        PrivateTmp = true;
        PrivateDevices = true;
        ProtectSystem = "strict";
        ProtectHome = true;
        ProtectKernelTunables = true;
        ProtectKernelModules = true;
        ProtectControlGroups = true;
        RestrictAddressFamilies = [ "AF_INET" "AF_INET6" ];
        RestrictNamespaces = true;
        LockPersonality = true;
        MemoryDenyWriteExecute = true;
        SystemCallArchitectures = "native";
      };
    };
  };
}

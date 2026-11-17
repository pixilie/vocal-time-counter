{ lib, stdenvNoCC, fetchurl }:

let
  release = lib.importJSON ./release.json;
  system = stdenvNoCC.hostPlatform.system;
  asset = release.assets.${system}
    or (throw "vocal-time-counter: no prebuilt binary for ${system} (publish a release first)");
in
stdenvNoCC.mkDerivation {
  pname = "vocal-time-counter";
  inherit (release) version;

  src = fetchurl { inherit (asset) url hash; };

  dontUnpack = true;

  installPhase = ''
    runHook preInstall
    install -Dm755 $src $out/bin/vocal-time-counter
    runHook postInstall
  '';

  meta = {
    description = "Discord bot tracking time spent in voice channels";
    homepage = "https://github.com/Pixilie/vocal-time-counter";
    license = lib.licenses.mit;
    mainProgram = "vocal-time-counter";
    platforms = builtins.attrNames release.assets;
    sourceProvenance = [ lib.sourceTypes.binaryNativeCode ];
  };
}

{ stdenv
, egiraffe-ng-frontend
, egiraffe-ng-backend
}:
stdenv.mkDerivation {
  name = "egiraffe-ng";

  installPhase = ''
    cp -r ${egiraffe-ng-backend} $out
    chmod -R u+w $out
    cp -r ${egiraffe-ng-frontend} $out/frontend
  '';

  dontBuild = true;
  dontUnpack = true;
  dontConfigure = true;
}

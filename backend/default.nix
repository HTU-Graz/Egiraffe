{ lib
, rustPlatform
, pkg-config
, openssl
}:

rustPlatform.buildRustPackage rec {
  name = "egiraffe";

  src = ./.;

  cargoHash = "sha256-RxcdzCKSuHI1ZJesnXZbesXBmelJQRb21P2d1vxq7sQ=";

  buildInputs = [
    openssl
  ];

  nativeBuildInputs = [
    pkg-config
  ];

  meta = with lib; {
    description = "A website to download and share exam papers and other study materials.";
    homepage = "https://gitlab.tugraz.at/htu/egiraffe/egiraffe-ng";
    license = licenses.agpl3Plus;
    # maintainers = [ ];
  };
}

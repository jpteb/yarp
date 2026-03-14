{ ... }:
{
  perSystem =
    {
      pkgs,
      config,
      ...
    }:
    let
      crateName = "yarp";
    in
    {
      # declare projects
      nci.projects."yarp" = {
        path = ./.;
        depsDrvConfig.mkDerivation = {
          nativeBuildInputs = [
            pkgs.cmake
            pkgs.pkg-config
            pkgs.perl
          ];
        };
      };
      # configure crates
      nci.crates.${crateName} = {
      };
    };
}

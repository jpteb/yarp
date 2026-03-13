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
      nci.projects."yarp".path = ./.;
      # configure crates
      nci.crates.${crateName} = {
      };
    };
}

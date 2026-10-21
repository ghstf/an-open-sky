{
  pkgs,
  ...
}:

{
  packages = [ pkgs.cargo-about pkgs.jq pkgs.just pkgs.zlib ];

  languages = {
    rust = {
      enable = true;
      channel = "stable";
    };
  };
}

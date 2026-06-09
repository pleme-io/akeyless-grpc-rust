{
  description = "Akeyless gRPC Rust library for interacting with the Akeyless API";

  inputs = {
    nixpkgs.follows = "substrate/nixpkgs";
    substrate = {
      url = "github:pleme-io/substrate";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs:
    (import "${inputs.substrate}/lib/repo-flake.nix" {
      inherit (inputs) nixpkgs flake-utils;
    }) {
      self = inputs.self;
      language = "rust";
      description = "Akeyless gRPC Rust library for interacting with the Akeyless API";
    };
}

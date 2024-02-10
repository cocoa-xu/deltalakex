defmodule DeltaLakex.MixProject do
  use Mix.Project

  @version "0.1.0-dev"
  @github_url "https://github.com/cocoa-xu/deltalakex"
  @dev? String.ends_with?(@version, "-dev")
  @force_build? System.get_env("DELTALAKEX_BUILD") in ["1", "true"]

  def project do
    [
      app: :deltalakex,
      version: @version,
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      description: description(),
      package: package(),
      source_url: @github_url,
      aliases: [
        "rust.lint": ["cmd cargo clippy --manifest-path=native/image_rs/Cargo.toml -- -Dwarnings"],
        "rust.fmt": ["cmd cargo fmt --manifest-path=native/image_rs/Cargo.toml --all"],
        ci: ["format", "rust.fmt", "rust.lint", "test"]
      ]
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      {:castore, "~> 0.1 or ~> 1.0"},
      {:rustler, "~> 0.30", optional: not (@dev? or @force_build?)},
      {:rustler_precompiled, "~> 0.7"},
      {:ex_doc, "~> 0.29", only: :docs, runtime: false}
    ]
  end

  defp description() do
    "A tiny Elixir library for image decoding task with image_rs as the backend."
  end

  defp package() do
    [
      name: "deltalakex",
      files: ~w(
        lib
        native
        checksum-*.exs
        mix.exs
        README.md
        LICENSE),
      licenses: ["Apache-2.0"],
      links: %{"GitHub" => @github_url}
    ]
  end
end

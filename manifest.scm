(use-modules (guix packages)
             (gnu)
             (gnu packages rust)
             (gnu packages tls)
             (gnu packages pkg-config))

(packages->manifest (list rust
                          (list rust "cargo")
                          (list rust "tools")
                          rust-analyzer))

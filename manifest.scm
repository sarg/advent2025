(use-modules (guix packages)
             (gnu)
             (gnu packages rust)
             (gnu packages maths)
             (gnu packages tls)
             (gnu packages pkg-config))

(packages->manifest (list rust
                          lpsolve
                          (list rust "cargo")
                          (list rust "tools")
                          rust-analyzer))

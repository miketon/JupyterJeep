[profile.release]
strip = true
opt-level = "z"
lto = true
panic = "abort"

toml settings
- to reduce binary size by 93% from ~ 3mb to ~ 268kb

main.rs
- manual deport and min to reduce binary size further from ~ 268kb to ~ 14kb
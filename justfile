release := '--profile release'
dev     := '--profile dev'

default:
    just --list

test:
    cargo test {{dev}}

check:
    cargo clippy
    cargo build {{release}}
    cargo test {{release}}

confirm-publish:
    @ read -r -p "Publish these files? (yes/no) " answer; [[ $answer == "yes" ]]
    @ echo "DRY RUN ========================"
    @ cargo publish --dry-run
    @ just files-in-crate
    @ read -r -p "Are you sure? (type in the crate name): " answer; [[ $answer == "macrobat" ]]

files-in-crate:
    @ echo "================================"
    @ cargo package --list
    @ echo "================================"

publish:
    @ just files-in-crate
    @ just confirm-publish
    cargo publish

$version = $args[0]

cargo bump $version
# Force Cargo.lock to be updated as well
cargo check

git cliff --tag $version -o CHANGELOG.md

git add .
git commit -m "Release $version"
git tag $version -m "Version $version"

echo "Done. If the created commit looks good, git push --follow-tags to finish."

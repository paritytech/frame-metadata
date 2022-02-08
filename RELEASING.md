# Release Checklist

These steps assume that you've checked out the `frame-metadata` repository and are in the root directory of it.

We also assume that ongoing work done is being merged directly to the `main` branch.

1. Ensure that everything you'd like to see released is on the `main` branch.

2. Create a release branch off `main`, for example `release-v14.3.0`. Decide how far the version needs to be bumped based
    on the changes to date. If unsure what to bump the version to (e.g. is it a major, minor or patch release), check with the
    Parity Tools team.

3. Check that you're happy with the current documentation.

    ```bash
    cargo doc --open --all-features
    ```

    Optionally you can also confirm that any external links
    are still valid like so:

    ```bash
    cargo install cargo-deadlinks
    cargo deadlinks --check-http -- --all-features
    ```

    If there are minor issues with the documentation, they can be fixed in the release branch.

4. Bump the crate version in `frame-metadata/Cargo.toml` to whatever was decided in step 2.

5. Update `CHANGELOG.md` to reflect the difference between this release and the last. If you're unsure of
   what to add, check with the Tools team. See the `CHANGELOG.md` file for details of the format it follows.

   Any [closed PRs](https://github.com/paritytech/frame-metadata/pulls?q=is%3Apr+sort%3Aupdated-desc+is%3Aclosed) between the last release and
   this release branch should be noted.

6. Commit any of the above changes to the release branch and open a PR in GitHub with a base of `main`.

7. Once the branch has been reviewed and passes CI, merge it.

8. Now, we're ready to publish the release to crates.io.

    Checkout `main`, ensuring we're looking at that latest merge (`git pull`).

    ```bash
    cd frame-metadata && cargo publish
    ```

9. If the release was successful, tag the commit that we released in the `main` branch with the
   version that we just released, for example:

    ```bash
    git tag -s v14.3.0 # use the version number you've just published to crates.io, not this one
    git push --tags
    ```

    Once this is pushed, go along to [the releases page on GitHub](https://github.com/paritytech/frame-metadata/releases)
    and draft a new release which points to the tag you just pushed to `main` above. Copy the changelog comments
    for the current release into the release description.

# Release Notes

This release includes:

- Mood Mosaic interactive web app
- Custom mood submission (POST /api/custom) with simple JSON persistence
- Additional moods and improved UI visuals
- Unit tests and CI workflow
- Local end-to-end check script in `tools/test_end2end.sh`

To create a GitHub release:

1. Tag the current commit:

```bash
git tag -a v1.0.0 -m "Mood Mosaic v1.0.0"
git push --tags
```

2. Open the repository on GitHub and create a release from the tag.

# Activity Action

This is a github action that automatically pulls the contributor statistics for your repository
and visualize it in your RAEDME file.

# Contributors

## Project 2 (2024-01-20-2024-03-29
![](https://github.com/aoli-al/activity-action/blob/main/example/image.svg)
<details>
    <summary>Click to expand!</summary>
<table><tr>
<th>
    <table>
        <tr>
            <th>
                <table>
                    <tr>
                        <th>
                            <a href="https://github.com/aoli-al">
                                <img src="https://github.com/aoli-al.png" alt="1" width=100px height=100px>
                            </a>
                        </th>
                    <tr>
                    <tr>
                        <th>
                            <a href="https://github.com/aoli-al/activity-action/commits?author=aoli-al">aoli-al</a>
                        </th>
                    <tr>
                </table>
            </th>
            <th>
                <table>
                    <tr>
                        <th align="left">
                            Commit: 29
                        </th>
                    <tr>
                    <tr>
                        <th align="left">
                            Addition: 64596
                        </th>
                    <tr>
                    <tr>
                        <th align="left">
                            Deletion: 336
                        </th>
                    <tr>
                    <tr>
                        <th align="left">
                            Issues: 0
                        </th>
                    <tr>
                    <tr>
                        <th align="left">
                            PRs: 0
                        </th>
                    <tr>
                    <tr>
                        <th align="left">
                            Comments: 0
                        </th>
                    <tr>
                </table>
            </th>
        <tr>
    </table>
</th>
</tr></table></details>




# Instructions

1. Create a new Github workflow in your repository: `.github/workflows/activity-action.yml`.

```yml
name: Update README

on:
  workflow_dispatch:
  push:

jobs:
  activity-action:
    runs-on: ubuntu-latest
    permissions:
      contents: write

    steps:
      - uses: actions/checkout@master

      - uses: aoli-al/activity-action@main
        env:
          GITHUB_TOKEN: ${{ secrets.PERSONAL_GITHUB_TOKEN }}
        with:
          template: "README.md.tpl"
          output: "README.md"

      - uses: stefanzweifel/git-auto-commit-action@v5
        with:
          commit_message: Update generated README
          branch: main
          commit_user_name: activity-action 🤖
          commit_user_email: actions@github.com
          commit_author: activity-action 🤖 <actions@github.com>
          file_pattern: 'README.md'
```

Here you may want to modify the **template** and **output**. **template** is the template README file with placeholder <code>{-Activity<foo></foo>Location-}</code> where the contributor table will be shown. You can find an example [here](https://github.com/aoli-al/activity-action-example/blob/main/README.md.tpl). **output** is the path to the generated readme file.

Also this will run action everytime you **push** to Github. You can change it to different [triggers](https://docs.github.com/en/actions/using-workflows/workflow-syntax-for-github-actions#on).

2. You need to create a Github token as a secret called `PERSONAL_GITHUB_TOKEN`. You can create a new token by going to your profile settings: `Developer settings` > `Personal access tokens` > `Tokens (classic)` > `Generate new token`. You do **NOT** need to select any scopes.

3. Add the generated Github token to your repository secrets: `Settings` > `Secrets and variables` > `Actions` > `Secrets` > `New repository secret`. Name: `PERSONAL_GITHUB_TOKEN`; Value: `YOUR_NEW_TOKEN_VALUE`.

4. You can find an example [here](https://github.com/aoli-al/activity-action-example);

# Issues

GitHub Issues in `Pissmyran/bastion-linux` is the repo's issue tracker.

## Working with issues

- Read the whole issue before working on it: text, labels, comments and linked pull requests.
- An issue should describe one coherent problem, with acceptance criteria and blockers in the text.
- Decisions that should last go into the documentation, not only into a comment.

## From a cloud session

`gh issue` does not work in cloud sessions (GraphQL is disabled). Use REST:

- Read: `gh api repos/Pissmyran/bastion-linux/issues/<n>` and `…/issues/<n>/comments`
- List open: `gh api 'repos/Pissmyran/bastion-linux/issues?state=open'`
- Create: `gh api repos/Pissmyran/bastion-linux/issues -f title='…' -f body='…'`
- Comment: `gh api repos/Pissmyran/bastion-linux/issues/<n>/comments -f body='…'`
- Close: `gh api -X PATCH repos/Pissmyran/bastion-linux/issues/<n> -f state=closed`

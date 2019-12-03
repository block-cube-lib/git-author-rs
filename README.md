# git-author

[日本語ドキュメント](https://block-cube-lib.github.io/documentation/git-author/)

## Overview
- You can get or set user.name and user.email at oece.
- You can replace the author of past commits.

## Description
You can get user.name and user.email with `git author get` or `git author`.  
![get-demo](/media/get.gif)

---

You can set user.name and user.eamil with `git author set foo foo@abc.com`.  
![set-demo](/media/set.gif)

---

Replace `foo <foo@abc.com>` in author and commiter of previous commits with `bar <bar@xyz.com>`.  
The replacement name and e-mail address (`bar bar @ xyz.com`) can be omitted.
If omitted, it will be replaced with author which can be obtained by` git author get`.
![replace-demo](/media/replace.gif)

## Installatoin
`cargo install git-author`

## License
MIT

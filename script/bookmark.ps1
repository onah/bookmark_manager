$selection = bookmark_manager.exe list | fzf.exe
$number = $selection -replace '^(\d+) \| .+', '$1'
bookmark_manager.exe execute $number
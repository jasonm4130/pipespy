# pipespy demo recording
Output assets/demo.gif
Output assets/demo.webm

Set Shell "bash"
Set Theme "Catppuccin Mocha"
Set FontFamily "JetBrainsMono Nerd Font"
Set FontSize 16
Set Width 1400
Set Height 800
Set Padding 24
Set TypingSpeed 50ms
Set PlaybackSpeed 1

# Show the command
Type "cat /tmp/pipespy-demo.jsonl | pipespy"
Sleep 800ms
Enter

# Let compact mode render with data flowing
Sleep 4s

# Toggle to fullscreen
Type "f"
Sleep 5s

# Back to compact
Type "f"
Sleep 2s

# Quit
Type "q"
Sleep 1.5s

# Show quiet mode
Type "cat /tmp/pipespy-demo.jsonl | pipespy -q > /dev/null"
Sleep 500ms
Enter
Sleep 3s

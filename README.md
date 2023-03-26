# WormtailBot
A multithreaded twitch bot 

Not done yet, i've been working on this for a couple hours to understand how the crate works with threads.
i'll maybe make it so the controller can turn on / off instances of the bot (for multiple channels)

Atm, the bot works, you can control it by typing commands in the console or in twitch chat

Update:
restructured the code and worked a bit on a app system that manage both the client side controller and the bot
could be upgrded to manage more bots with some cd-like command to switch bot and manage what bot can output data

controller's commands: (in console)
quit / exit to kill the bot and the app
ban user[string, mendatory] reason[string, optional]
unban user[string, mendatory]
say message[string, mendatory]

bot's command: (in twitch chat) bot prefix is *
uptime      // tells the uptime of the channel
say         // repeat the next words
disconnect  //disconnects from the twitch chat and closes the app
help        // send a help message NOT DONE YET

and whenever someone says "gaming" it responds to it by a "gaming"


Todo:
clean a bit the raw api calls


How to use:
create a file named "t.ron" in /config, follow the "t.ron.example"


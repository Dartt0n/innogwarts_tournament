# Innogwarts tournament

Harry Polis, Hermione Grannopark, Ron Weasinno and others from Hogwartolis applied to Innopolis University and got accepted, and when they reached Innopolis they saw people coding and doing things that are similar to magic like solving assignments and exercises from ITP course. So, they decided to teach the students magic and then they created their own club for magic. However, friends were unable to choose the club head and decided to ask other students for assistance. For this students created teams and each team chose their unique favorite magician. The teams decided to compete between each other by playing a duel with multiple teams playing simultaneously. The winning team's choice of the club head should become the solution for the game. Help students to play the game by using mysterious C code. Let the magic begin!


In the input.txt file you will have the following content separated by a new line:

1. The number $N$ ($1 \le N \le 10$), which is the number of teams in the game.

2. $N$ lines, each line should contain a unique magician name with length $L$ ($2 \le L \le 20$) made of only English letters and should begin with capital letter. Each of those magician names should correspond to the team number from $0$ till $N−1$, which was chosen by this team to become the head of the club.

3. The number of the players $M$ ($N \le M \le 100$). After this you will have $M*4$ lines, each line will represent a $player_i$ information:
    - The unique name of the player $name_i$ with length $L$ ($2 \le L \le 20$) should contain only English letters and begin with capital letter.
    - Team number for this player $t_i$ ($0 \le t_i < N$).
    - The power of the player $p_i$ ($0 \le p_i \le 1000$), which is integer value.
    - The visibility of the player $v_i$ ($True$/$False$) 

4. Sequence of actions for players $S$ ($0 \le S \le 1000$) followed on the same line by 1 or 2 player names separated by single spaces. Not each player is guaranteed to have actions. Next actions are applicable:

    - $attack$ $name_i$ $name_j$
        - if $p_i>p_j$, then playeri will gain $p_i−p_j$ power and $player_j$ will have no power left and further will be called frozen.
        - if $p_i<p_j$, then playeri will be frozen and $player_j$ will gain $p_j−p_i$ power.
        - if $p_i=p_j$, then both players will be frozen.
        - if $player_j$ is not visible, then the $player_i$ will be frozen. 

    - $flip_visibility$ $name_i$ will flip the status of the visibility of the $player_i$.
    - $heal$ $name_i$ $name_j$ will make $player_i$ give half (ceil the numbers up for both players if needed) of his power to $player_j$ (from the same team).

    - $super$ $name_i$ $name_j$ will create a super player instead of existing 2 players (from the same team) with joint power and actions. The power will be $p_i+p_j$ (sum at most 1000, a greater sum should be floored down to 1000) and the visibility will be $True$ and the name assigned to super player will be `S_k`, where $k$ is the index of super player ($k$ begins with 0 and increments for each next new created super player).

A frozen player can be healed, then this player will be unfrozen.

A player with visibility equal to $False$ can be healed.

A frozen $player_j$ can be in the action in $super$ $name_i$ $name_j$.

A $player_j$ with visibility equal to $False$ can be in the action in $super$ $name_i$ $name_j$.

You might have a team with zero players, but it is guaranteed that all players have assigned teams.

The cases when you cannot perform the actions, but they should not stop the game and the actions should be ignored:

1. If a player with the visibility equal to $False$ tries to make any action other than $flip_visibility$, then the next warning message should be added to the output file: `"This player can't play"`.

2. If a frozen player tries to make an action, then the next warning message should be added to the output file: `"This player is frozen"`.

3. In case of $heal$ or $super$ action, if players are from different teams, then the next warning message should be added to the output file: `"Both players should be from the same team"`.

4. The player shouldn't be able to heal itself, in this case in the output file the next warning message should be added: `"The player cannot heal itself"`.

5. If during creation of a super player the $player_i$ unites with $player_i$, then the next warning message should be added to the output file: `"The player cannot do super action with itself"`.

If multiple described errors happen for a single action, print ONLY the first one of them in the order given above.

The input file has to be read until the end and all actions have to be performed sequentially.

At the end of the output file the next message should be added: `"The chosen wizard is #team's_wizard"` (the wizard of the team which has the greatest power left which is the sum of all the team members' powers). If more than 1 team has the greatest power the output file should contain a different message, which is: `"It's a tie"`.

The `input.txt` file may contain invalid inputs, which has to be detected and reported in the `output.txt` file with the only error message `"Invalid inputs"`. If the output file already contains some other previous messages, everything has to be replaced by an error message.

P.S.: It is guaranteed that magicians' and players' names are not intersecting in the same test, i.e. you should not expect the input with the player and magician having the same name.

P.P.S.: You have to use structure(s) for this assignment
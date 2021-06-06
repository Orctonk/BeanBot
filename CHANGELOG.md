# Changelog

## Version 0.3.0 *Sprout*
* Added special beans which can be acquired from bean jars which can be bought for beans
    * The user can use the **mybeans** command to see their special beans and the **about** command to see information about one of the beans.
    * Added the base for an admin page for the special beans images used
* Now uses [rust-clippy](https://github.com/rust-lang/rust-clippy) to lint the rust source

## Version 0.2.1 *Sprout*
* Added score board and leader of scoreboard.
    * **beanmaster** prints the name of the user with the most beans.
    * **beanboard** prints the top 10 bean bois.
* Added functions in SQL currency module to fetch users. 

## Version 0.2.0 *Sprout*
* Added the translation module:
    * **translate** translates the provided text to english or to a provided language from a source or detected language base on parameters.
    * **detect** guesses the language of the provided text.
    * Backend provides utilities for authorizing against Google's Translate API.
* Added a settings ini to allow for easier expansion without having to pass multiple parameters to the bot.
* Bug fixes in wallet module.

## Version 0.1.3 *Sprout*
* Added some new commands in the currency module:
    * **eat** lets the user eat some beans.
    * **daily**, **weekly**, **monthly** and **yearly** gives the user beans but has a cooldown time.
* Made the backend currency module more robust and expandable by adding custom error enums which each correspond to an error which can occur and optionally provides additional information

## Version 0.1.2 *Sprout*
* Updated documentation for currency module
* Added some error messaging on failed dispatches
* Added simple help command

## Version 0.1.1 *Sprout*
* Removed redundant imports.
* Fixed indexing bug in currency module which caused crashes when issuing give command without a mention.

## Version 0.1.0 *Sprout*
* Added currency backend using SQLite.
* Added frontend for currency module using prefix **beans**:
    * **gimme** gives the user beans.
    * **showme** displays current bean balance.
    * **give** gives the mentioned user the specified amount of beans.
* Previous **beans** command is now **showmebeans**.

## Version 0.0.1 *Sprout*
* Initial version with basic functionality.
* Basic project structure with serenity framework.
* **beans** command which prints beans in the chat when issued.

# Changelog

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

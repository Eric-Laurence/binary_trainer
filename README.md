# What?

This is a simple tool for generating flashcards with random numbers and their conversion into different bases (binary, hex, etc.)

# Why?

It's useful if you want to learn how to read binary, hex, or base64 values.

The idea for creating this came from the Binary Racer minigame in Turing Complete, where you have to convert a decimal number into a binary number up to the size of a byte. There is an achievement for reaching the last level of this minigame, but it gets quite difficult towards the end and the only viable way to win is to be able to convert decimal to binary in your head.

# How?

Select what base you want the front of the flashcard to be in (input base) and what base you want the back of the flashcard to be in (output base). Enter decimal values (e.g. 5) or ranges of values (e.g. 5-27) in the input box, separated by commas. Click "Generate" to get a new random value from the values and ranges in the input box, and click "Check" to see the value converted into whatever base the output base is set to.

You can optionally pad the input and output, which is mainly useful for padding binary values up to a byte, or padding hex values up to 2 characters (for reading color values).

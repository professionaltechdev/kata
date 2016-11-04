# Trigrams

http://codekata.com/kata/kata14-tom-swift-under-the-milkwood/

Trigram analysis is very simple. Look at each set of three adjacent words in a
document. Use the first two words of the set as a key, and remember the fact
that the third word followed that key. Once you’ve finished, you know the list
of individual words that can follow each two word sequence in the document. For
example, given the input:

```
I wish I may I wish I might
```
You might generate:

```
"I wish" => ["I", "I"]
"wish I" => ["may", "might"]
"may I"  => ["wish"]
"I may"  => ["I"]
```

To generate new text from this analysis, choose an arbitrary word pair as a
starting point. Use these to look up a random next word (using the table above)
and append this new word to the text so far. This now gives you a new word pair
at the end of the text, so look up a potential next word based on these. Add
this to the list, and so on...

See the main article for more details!


# Tasks

* Using the dictionary listed above ("I wish I may I wish I might"), select a random
key as the start of your sentence, and use the dictionary to attach a word to it.
* Loop through this process with the final 2 words in your sentence each time
to extend the sentence. Stop the loop when you reach a key that does not exist
in your dictionary.
* Build a simple algorithm that analyses a piece of text and builds a data
structure containing your trigrams from it.
* Apply your loop from the first few tasks to that text to produce a new story.
* Post your best stories in slack!

Bonus:

* Feed your altgorithm from a file containing a large amount of text, e.g. a book from Project Gutenberg.
* Automate feeding the algorithm with more books from Project Gutenberg.

Got an idea for a bonus task? Fork the repo and add it!


# Submitting an entry for this kata

All languages are welcome. This kata exists purely to exercise your brains -
don't feel obliged to write concise code, or lengthy code, or clever code. Just
do whatever makes you happy and stretches your brain.

If you'd like people to review your code, you are very welcome to fork this repo
and open a pull request, adding the code into a subfolder beneath this one
with the same name as your github username. For example, I'm `veryhappythings`,
so I'd put my code in `kata1/veryhappythings`.

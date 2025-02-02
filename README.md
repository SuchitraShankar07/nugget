# nugget
`Learning project, fuzzy searches my phone directory to find contacts who's names are mapped to keypad configurations`



This works by initially mapping the number on T9 keypad to appropriate letters and generates a list of possible names, proceeds to fuzzy match with the contact list, could optionally greedy fuzzy match with a smaller time complexity of O(n).

---
What it uses:
> nucleo = "0.1"
csv="1.3"
serde="1.0"
---
Has absolutely no purpose except to help me get more familiar with the language while also not losing my mind reading the rust book.
Had lots of fun making this :)
Apologies for the redundant code

---
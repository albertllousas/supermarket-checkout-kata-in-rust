# checkout kata in rust

Code kata inspired in the [checkout kata](http://codekata.com/kata/kata09-back-to-the-checkout/).

**Goal:** Learn some Rust.

## Description

Implement the code for a supermarket checkout that calculates the total price of a number of items. 

### Rules

- Our checkout accepts items in any order, to do so, we can scan items, each scan we will get either the price
or and error 
- For each successful scan, we will add the item to the current list of products.
- We should be able to remove items from the checkout.
- Any time we can calculate the total, it will sum up the total price of the products successfully scanned

```
Item      Unit      Special
Price     Price
--------------------------
A        50         3 for 130
B        30         2 for 45
C        20
D        15
```

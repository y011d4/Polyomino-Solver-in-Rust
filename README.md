# Polyomino solver
### How to use
##### Create pieces
Pieces can be created by `Piece::new` function.
```
impl Piece {
    pub fn new(s: &str, nrp: usize, ncp: usize, rot: bool, rev: bool, ncb: usize) -> Piece {
```
s: string describing shape of piece

nrp: row size of the piece

ncp: column size of the piece

rot: whether the piece can be rotated or not

rev: whether the piece can be reversed or not

ncb: column size of the board

###### Example

xxxxxx  
xooxxx  
xxooox  
xxxxxx  
xxxxxx  

The block of piece is displayed as 'o'.
This piece is described in string as '11000111'.
To solve, the piece position should be written in one dimensional array.


That is, if you want to create this piece, you can make it by  
`Piece::new('110011001', 2, 4, true, true, {the column size of board})`

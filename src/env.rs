use std::fmt;
use std::collections::HashSet;
use std;

pub struct Board {
    pub field: Vec<isize>,
    pub nr: usize,
    pub nc: usize,
}

pub struct Piece {
    pub pos: Vec<Vec<usize>>,
    pub used: bool,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for _i in 0..3*self.nc+3 {
            write!(f, "-").unwrap();
        }
        write!(f, "\n").unwrap();
        for (i, elem) in self.field.iter().enumerate() {
            if i%(self.nc+1)==0 {
                write!(f, "| ").unwrap();
            }
            if i%(self.nc+1)==self.nc {
                write!(f, "|\n").unwrap();
            } else {
                write!(f, "{: >2} ", elem).unwrap();
            }
        }
        for _i in 0..3*(self.nc+1) {
            write!(f, "-").unwrap();
        }
        write!(f, "\n")
    }
}

impl Board {
    pub fn new(nr: usize, nc: usize) -> Board {
        let mut field: Vec<isize> = vec![0; (nc+1)*nr];
        for i in 0..field.len() {
            if i%(nc+1)==nc {
                field[i] = -1
            }
        }
        Board {
            field: field,
            nr: nr,
            nc: nc,
        }
    }
}

impl Piece {
    pub fn new(s: &str, nrp: usize, ncp: usize, rot: bool, rev: bool, ncb: usize) -> Piece {
        //let mut pos: Vec<Vec<usize>> = Vec::new();
        let p_size = std::cmp::max(nrp, ncp);
        let mut piece_2d: Vec<Vec<usize>> = vec![vec![0; p_size]; p_size];
        assert_eq!(s.len(), nrp*ncp);
        for (i, c) in s.chars().enumerate() {
            let c_int = (c as usize)-48;
            assert!(c_int==0 || c_int==1);
            piece_2d[i/ncp][i%ncp] = c_int;
        }

        //for _i_rev in 0..n_rev {
        //    for _j_rot in 0..n_rot {
        //        let mut tmp: Vec<usize> = Vec::new();
        //        for i in 0..p_size {
        //            for j in 0..p_size {
        //                if piece_2d[i][j]==1 {
        //                    tmp.push(i*(ncb+1)+j);
        //                }
        //            }
        //        }
        //        let mut res: Vec<usize> = Vec::new();
        //        for elem in &tmp {
        //            res.push(elem-tmp[0]);
        //        }
        //        pos.push(res);
        //        rotate(&mut piece_2d);
        //    }
        //    reverse(&mut piece_2d);
        //}
        let pos: Vec<Vec<usize>> = gen_all_patterns(&mut piece_2d, rot, rev, &ncb);
        Piece {
            pos: pos,
            used: false,
        }
    }
}

fn rotate<T>(v: &mut Vec<Vec<T>>) where T: Clone {
    assert_eq!(v.len(), v[0].len());
    let tmp: Vec<Vec<T>> = v.clone();
    let n: usize = v.len();
    for i in 0..n {
        for j in 0..n {
            (*v)[i][j] = tmp[n-1-j][i].clone();
        }
    }
}

fn reverse<T>(v: &mut Vec<Vec<T>>) where T: Clone {
    assert_eq!(v.len(), v[0].len());
    let tmp: Vec<Vec<T>> = v.clone();
    let n: usize = v.len();
    for i in 0..n {
        for j in 0..n {
            (*v)[i][j] = tmp[i][n-1-j].clone();
        }
    }
}

fn gen_all_patterns(piece_2d: &mut Vec<Vec<usize>>, rot: bool, rev: bool, ncb: &usize) -> Vec<Vec<usize>> {
    assert_eq!(piece_2d.len(), piece_2d[0].len());
    let n_rot = if rot { 4 } else { 1 };
    let n_rev = if rev { 2 } else { 1 };
    let p_size = piece_2d.len();
    let mut pos: HashSet<Vec<usize>> = HashSet::new();

    for _i_rev in 0..n_rev {
        for _j_rot in 0..n_rot {
            let mut tmp: Vec<usize> = Vec::new();
            for i in 0..p_size {
                for j in 0..p_size {
                    if piece_2d[i][j]==1 {
                        tmp.push(i*(ncb+1)+j);
                    }
                }
            }
            let mut res: Vec<usize> = Vec::new();
            for elem in &tmp {
                res.push(elem-tmp[0]);
            }
            pos.insert(res);
            rotate(piece_2d);
        }
        reverse(piece_2d);
    }
    let mut retpos: Vec<Vec<usize>> = Vec::new();
    for elem in &pos {
        retpos.push(elem.clone());
    }
    retpos
}


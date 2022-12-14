use serde_json::Value;
use std::cmp::Ordering;
use std::io;

fn main() -> Result<(), io::Error> {
    let data = r#"
        [
            [[[7,[6,4,3],4,[5,5,0]],[9,2],5],[[0,[3]],[[2,3,2,10,8],10,8,[2,7,6]],5,[[0,8,6,10],[2],[8,7,6,4,9]],10],[7,9,1,0,[]],[[[8]],[8,0,4,[8],[1]],4,[[9],4]]],
[[6,[[3,5,7,4],[6,5,7,10,5],1,[6,9],2]],[1,[]],[[[10,1],[],1,[9,0,1]],[[9,9],[7]]],[0,[6,9,[],10,2],[],[]],[8,6,6,6]],

[[6],[2],[5,9],[9]],
[[[0,[9,8,6]],[3,[]],8,1],[[6,[2],2,7]]],

[[3,7,[0,6,[5,10,0,4,8],[9,4,1,5,1],[1,1,7,0]],5],[6,[0,[],6,[7,4,7],[1,3]],2,1,3],[[6,6,[6],6],0,1,[[8,0,2,9],[]],[0,[2,1],5,[6,0,2,1],2]],[]],
[[[],[3],8,[2,[1,10]]],[5],[]],

[[],[[[10,4,7],[8,3],4,8]]],
[[[[9,3,4],[]],3,[],0],[],[[[6],1,[8,8],[]],[2,4,7],1,3]],

[[[[2,1,2,5],[10,1,6,8,7],[3],[9,2]],9,[1,8,9],8],[0,[9,3,[10,8,10,3,4],[8,10]]],[[[9,9],[2,5]],[[3],1,[2],10],9]],
[[],[[[7,1,10,2,2],3,[0,8,4,6],[3,4]],3,0,6],[],[2,[9,4,[],[]],[8],0]],

[[[7,3,[]],0],[3,[2]]],
[[[[10,1,7],[7]]],[[7,6,7]],[[3,[6,4],[9,9,5],[2,8,5,4]],10],[[[4,5,10,1,1],[4,10],[8,9],[6,8,1,6],9],[[8,9],4,[],1,1],[2,[6,0],10,[],8],2],[8,[[],2]]],

[[5,8,7],[]],
[[[8]],[6,[[3,0,5],[8,1,9,0,2],2,3],2,[[0,3,10],9,[]],[[7,3,9],6]],[5,[6],[],[[7,1,3,8,5],7],4],[[],4,[6,[10,6,10],[10,7,9]],[[]],7]],

[[[6,0,3,[9,10,4,7]],[4,8,3],10,3],[[1,[4,7,9,0],[7,1,1,1,5],4],2,4,0,7],[0,7,8,[0,7,[7,9],[8,6,4,8,3],0]],[10,4,9,5,[]],[[],[[4,0]],5,0,3]],
[[10,3,[[1,8,7,8],5,8,[4,3,3,4]],[6,[9,4,10,10],4,[3,9,9,10,2]]]],

[[],[[4,3,7,[7,5,1,7,4]],[],[[4],6,[0,0,0,2],5,6],[[9,10,5],[7,0,10,6],6,[10,0]],5]],
[[[[5]],[]],[[10,4,0],6],[[],[[8,2,1]],[9,[6,5,8,0,5],[3,2,3,9,4],1,[4]],[],5]],

[[[],[[5]],3],[[[8,8,2,0],[9,4,7]]],[2,3]],
[[[[8],2,[],[5,5,7],4]],[[6,1,6,[5,4,8,2]],[2,5],9],[5],[[6,10,5,[5]],7]],

[[],[1],[0,[[8,5,10],[],9,[3,4,1],8],8,7,0],[4,[1],0,1]],
[[[],[[1,1,3]],4,10],[[0,[9,5],[5,6,3],7],[[9]],[],[[6,5],[1,2,1,6]],2],[9,6],[6,9,9]],

[[9],[2,[[10,1,7],2,[0]]]],
[[6,9]],

[[9,[2,1],7,7,[8,5,[8,6,5],[1,3,2]]]],
[[[5,0,4],[],[]],[1,[2],[0,[7,8],[7]],4,[7,[8,5,4,2,10],9,[9,4,9,5,7],0]],[1,[[7],1,1,4]],[9,[5,[6,7,5,7,8]],6,6,2]],

[[],[],[[],10,[2,[]],0,[[10,0],6,[7],[6]]],[0]],
[[[8,2],10,2,[]],[[4,[9,4,7],4],4,9,5],[[[1,0,1],0,8,[]],7]],

[[0,[6,[6],[],1],4,0,7],[4,7,6,6,8],[2,[7,7],[[7,6,8,4,6],[6,10]],7]],
[[[9,4,[0,0],9]],[],[2],[[2,10,9,10],4,10,4,[6,[2,6,2,0,1],4]],[1,[[9,4]],[6,5],3]],

[[],[],[9]],
[[[[10,6]],3,9,[3]],[[[5,2,1],3],[]]],

[[[[]],[],[],[[],10,[2,2],[10,6],5],[10,[2,6,3,3],7,[5],[10,3,2,7,9]]],[7],[[3,2,[],10],[],[4,[9],[10,1,5]]]],
[[[],9],[7],[[3,[2,2]],[[6,2,1],0],6,[6,7,[]]],[2]],

[[],[[9,5,[0,6,8,3,2]],4],[9,[8,[5,1,0,2,8]],[[8,0,3,6],[5,3,8,10],10],8,5]],
[[[[6,0],[]],[8,[8,10,6,2],6,5],8],[2,[[],9]],[],[5,[10,8],3]],

[[2],[5,[[]],9,5,[[7],5]],[[7,[5],[6,10]],8]],
[[5,[[8],[0],2,3,8]],[9,[[]],9],[[[0,4],6,0,[4,3,9]],10,2,[[5,7,8,2,7],1,[9,9,1,7],1,[9,8]],[[9],8,[2],[0]]]],

[[5]],
[[9,4],[10,[[9,0,3]],5]],

[[5],[[[1,2,4,6],3,2,1],7,[[9,2,8],4,[1,0,5,9,4]]],[0,0,[[2,5]]]],
[[[4,7,[3,9,10],[7,6,7,10,3]],10,10],[[],1],[[[7,0,4,7,2],[2],[],3,[10,10]]],[[[1,5,7,4,2],[8,7,6,6,9]],10,10,1]],

[[[1,4,10,2,[9,6]]]],
[[10],[1,[[1,0,0,3],9,3,[],[]],[3,2,[5,1,3]],9,10]],

[[[],2,2],[[[3,8,9,1,2],[1,0,10],10,[9,1],1],[[3],8],9,[[8,10,1,7,5],[],5,[10]]],[[1,2,[6,5],[8,10,10,3,5],[]],[],[[5,0,3],1],[]],[[[10,3,2,7,1]],10,[1,0,0],[[5,9,8,5],[3,2,0],[2,5],[0,3,2,10]]]],
[[[[0,10,2,0],3,3],[8,[7,7,4]],[2,8],4]],

[[4,[1,1],5]],
[[[[1,2,3,4,5]],3,1,2]],

[[],[7,[[1]],1],[],[],[6,[[4,3],7],3,8]],
[[9,1,[10,[3,1,10],9,[4],[8]],9],[[[],5,[4,0,4,6,3]],[[9],2,[4,2,10],[3,4,2,5],[]],[[6],[],1,[]]],[],[]],

[[9,[[8,1,9,2,4]],[[4,0,9],[0,1,4,1,8],0,8,[7,5,4,9]],[]],[7,[],0,[4,5,[4,6],4,6]],[[[2,8,3,10,3],6,[6,8],[6,7,5]],[5,8,1],[[5]],[6,9,[1,3]],[5]],[[[4,1],[],9,[6,6,5,4,4],7],9,[],[4,2]]],
[[6,2,10,1],[]],

[[4,[3],[7,10],3,[6]],[10,[],[[],9,6,7],8,[]],[],[[[5,5,2]],7,[6,3,4,3,[6,0,10]],[5,4,[],10,8],[2]]],
[[[[6,9,2,9,0],5,[4,3],6,5],[[7,6,10,1],[6,1,7,1,8]],7,4,[2,9,9]],[7,8,[[5,7],4],[[],7],[[7,9],[6,10,4,10]]]],

[[[[],5,0,[]],2],[[[],6,[9,5,4,5]],[0,[0]],0],[4],[[5,[0,8,3,3,4]],[9,6,0,10],9,0,[]]],
[[4,[[],[0,3,0],[7],[4,0,3,5]],[[7,1],10,0,10],6],[1],[[[3,5,1,7]],7,[[4],[3],[10,7,3,7,3],10],9],[1]],

[[0,5,10,[5],[8,[]]],[[1,[2,10,10,9],[3,5,4],[10,1,8]]],[[8]],[[10,[]]]],
[[],[[[2,2,3,1],8,[2,1,3,9],0,0],2,[[0,7,2,1],[2,0],8]],[],[[[3,9,1,3,9],[1]],0,[],[[4,10,8],7],6]],

[[[[6],[6,9,5,10],[10,2,7,2],3],10,7,[6]],[3,9,5],[0],[[[8,0],2,[10,5,9,6],9,7],[[2,9,4,6],8],3,10],[[10,[7,0,8],[8,3],[10,8]]]],
[[8,8,[0,4,10,[10]],[9,5]]],

[[[8,[],3,[10,5,3]]],[9,[],[4,[5,1,2,9,10],6,5],3,4],[],[0,[10,6,7],[8],10]],
[[4],[10,[]],[4],[4,5],[]],

[[[6,10],[],1,[[6,6],[10,7,4,6],6,[],0]]],
[[],[[[],8,5,5],[[5,3,2],[9],[5],0,[]],[7,3,7,5],[[],0,[],4],[[2],[7,5],0,[2,10],1]]],

[[0,4,5,[[10,6,4,1],6,5],[1,7,[4,4,8,3]]],[1]],
[[9,0,3,2],[6,9,10,8],[0,7,[[0],2],[5,[1,9,4],[10,5,7],6]],[[6,[10,0,3,2],[8,2,8,2,10],[10,1,5]],[[2],3,3],5,1,4],[[],1,[],[[4,5,10],[8,8,8,1],[5,1,0,1,4],[4,8]],6]],

[[4,0,9,[],9],[2,3],[8],[[[1,9,1,0]],8,3,8],[3,[[10,8]]]],
[[5,6,[9,[5,10]]],[[],10],[]],

[[[[5,4]],[]],[[],2,[3,[]],5]],
[[2,8,[[0,3,5]]],[[[1],[10,5,1,9,0],1],6,6,[[7,8,7,7,5],7,2,5]]],

[[[10,[6,6],[8],[4,7,0],[8,10,8]],8],[[],3]],
[[10,[[9,10,0],2]],[],[6,[[],3,[0,5]],3,5],[[[9]],1],[[5,[9,0,4,9],[5,7,8]]]],

[[[]],[[[],[],[],4],[7,4,[9,1]],[[10,0,9,10,4],8,6,8],[[8,9],[3,9],2],[]],[[[5,7,6,8,4],[0,9,2],[10,0,7,0]],5,4,10,1],[[3,7,[5,6,6,5],[8,9,6,5,5]],4,10,2],[0,[[2,1],[5],2],5,7,[]]],
[[9,5,4,[[4,5,5,10],4,[]],[]],[[]],[[[10]]],[2,3,1,[[6,4],9,[]]]],

[[[7,[6,10,1],[9],[4,1,6,10,5],[0,3,6,8]]],[],[6,3,9]],
[[[7,10,[0,7,6,1],8,[3,1]]],[[[10],2],[[3,9]],3],[[2,[6,10,1],3,[9,4,1,1]],7],[0],[]],

[5,5,2,2,3],
[5,5,2,2],

[[[2],5,[[10,10,7],[0,4],[4,1]]]],
[[],[4,4,[[1],[4,1,4,3,7],4,[10,5],[7]],8,[[],2,[2,5,9],4,[8,8,8]]],[[[0,8,7],[7,10,6],[],3,[2,1,9,3]],6,[1]]],

[[[],[]],[[1,[5,0,5,2,8],[5,4,2],9,0]],[[],6,[[2,1],8,[5,5],[],2],4,6],[[6,[9],[]],9,[]],[9]],
[[10]],

[[],[],[[[6]]],[[9,[9],7,[6]],6,9,7,[[9,8,0,4,8],[5],5,8]],[]],
[[[[10,5,2],0,[8],[3,9,8,7],[7,7,4]],[]],[],[[[2,5],[6],7,4],[5,[8,9,4],[3],10,6]]],

[[1,1,10,2]],
[[[[3,6],9,[5,10],[]],0],[[1]]],

[[0,5,10],[5,3,[[0,7,8,8]],[[4,5],9,[8,3,3,10,8],1]],[[6,[1,3,5,4]]],[0,8],[]],
[[],[6,5,9,4,1],[]],

[[[[6,7,1],7,[9,8,9,6],6,2],[[7,3],4,5],[1],6]],
[[],[7,[[1],[8,8],0,[7,7]],3,[[8,7],3],6]],

[[],[[2,[7,5,8],[8,5]]],[[],[],[9,[3]],[[1,7,10],[4]]],[[9,3],[9]],[3,[8,[1,2],0,10,[1,7]],[[10,10,8,2],[9],3,[2]]]],
[[[[9],[1,1,8,3],[0,5,10,8]],[0,1,[10,1,4,0],7,[10,10,9]],8]],

[[6,[[5],1,[4,4,1,5],6],8],[]],
[[3,5,9,8],[0,6,9],[2,[[10,5,5],1]]],

[[7,[[2,3,4,10],[5,0],[7,3,9,1],6,7],[2,1,[7,10,4,1,8],[],1],5],[4],[9]],
[[],[3,[[9,5]],[1,[5,6,7],[]],7,[[10]]],[[8,5,9],[],[],[[],5,[4,10]],1]],

[[2],[7,8],[[8,8,[6,1,8],5],2,5,[],[8,[0,5],6,[1,7,8,10]]],[[[4,7,0],5],[[7,4,5,7,1],[10],[8,2,10,3,7],[8,7],8]],[[],6,[],6]],
[[7,0,[[5,9,4,5,2],8,5,10],6]],

[[0],[[[0,9,8,0,2],9,[],10,5],[[10,3],0,10,[0,6,6,10],[10,0]],[5,[9],5,[1,6,3],10],[]]],
[[[0,2],2,7,5],[],[5,[6,7]]],

[[[[4],8],3,[],[5,[9,1]],2],[2,0,3]],
[[],[0,[[1,10,6,3],6,0,5],[[6],8,[3,5],[7,4],[5]]],[0],[[[],[4,8,9,2,2],9,[9,0]]],[]],

[[[[8,3]],[8,[6,7,1],8,[7,2,8,8,2],2]],[2],[4,[[],0,9],0,1],[[[0,5,2],7]]],
[[6,8],[[[5],7,[],10,3],[[7,0,3,9],[9,7,2],[4,10,8],[10,6]],[[],4,[10],4,3],4],[[],3,6,5,[3,1]],[1,8],[]],

[[],[4,2,[8,[10,10,8],1,4],[[10,10,1],[9],[10,8,5,3],10,2]],[[9,3,4,[4,1,8,7]],[[4,5,1,7],6,9,[]],0,[],[[6,0,5],4]],[[2,[],[3,10],10]]],
[[[[9],8,3,10],10,[[],[],[2,5,9,10,6],[],[9,3,6,8,9]],2,[10,3,0]],[10],[[[9,8,8,10,0],2],2]],

[[2,5],[[[4,8],6,[5]],[],[10,[6,7,6,3],[6,5,10]]],[7],[2,[0,3,5,0],[10]]],
[[7,4,6,[[4,8,3,9,0]],10],[],[[3,3,2,6,[]],[[4,4,4],[10,1]],9,[[5,4,2,6,10]],[[3,7,8,0],[6,7,3,6],3,5]]],

[[7,6],[8],[0]],
[[4,[[10,2],[7,3,0,6],[2,4,2,8,1],[4,2,0,9],[3,9,1,10]],10]],

[[[7,7,[2,8],7,9],[9,[7,10,6,5,6],[7,1,4],8],7,[[10],[5,0,3,10,7],[5,5,8,3,1]]],[0,[8,3,[9,1,1,8,8]]],[[2,[3,6,0,6,7]]],[[],[[4,8,4],2,3,[5],[6]],10,0,2],[]],
[[[[],0,9,[10,4],[1,9,3,2,4]]],[4],[[[2],[4,10,10]],0,[[3,7,7,1],5,[9,3,9,9,5],[8,1,8]]],[10,[[3],10,[],9],[[5],5,4,5,5],[[2],[10,1,7]]],[]],

[[6,[2,[8,8],[7,6,0]],8,[[3,6,5]],[[7,9,6],8,1,3,5]]],
[[[[1]],2,0],[[[9],[8,1,3,6,7],[7,4,4,1,4],[3,1,0],5],4,[10,[],3,5,0],[],[[9],[4,1,1,1,7],[3,9,0,2],0,[]]]],

[[[9,[9,7]],9,8],[7,10,[[3,2]],[[0,10,6,10,3]],[[8,3,8],3,[10,0,0,0],[1],[]]],[]],
[[[[10,5,7],[5],[7,5,3,10]],6],[]],

[[[10,[8,2,6],6,[]]],[3,8,[[],1,5,[1,0,6,0]],1]],
[[[[0,9],7],[[7],[]]],[],[[[1]],1,[7,1],8],[5,4,[1,0,[2,10],[]]]],

[[[4,3,4],[8,[9]],[3]],[[[4,0,5,2,3],2,[8,10,8],1],[[1,7,10],[3,10]],[2,[3,5]],[2,[10],8,8,2]],[7,4,[0,[8,6,7],[5,4],[2,0]]],[[4,3,[]],[7,9,5]],[]],
[[[0,[]]]],

[[[10,0,[],[9,1,5,5]],[1,1],0,4],[2,[5,7]],[[[4,3],[7,8,7,0],[3,8,5,0],3],5,[[]],3,7],[5,[[3,5,2]],3]],
[[2],[[],1],[10,5],[[[5],[3,4,6],9],[[2,7,9,8]],8],[[[8,2,7,10],9,5,[],0],2,[[7,1,0,2,8]],8,10]],

[[[[7,7,2,0,8],[],[10,9],5,4],2],[],[1]],
[[3,2,0]],

[[[0,4,4]]],
[[[[9],6,2,9,9],3,[[2],[0,9,5,10,9],8,[4,5]]],[[[4,0,5,3,1],8,[5,4,8,3],2],4,8,[[3,10,1,2],[0,9,7,3,2],[10]]],[],[6,0,[],7]],

[[4,0],[8,[[1,10,0,2]]],[[4,[3,3,1,9]],[],[3,0,[6]],[0,8,10,3]],[[],10,[2,4,[10,10],0,8],3,3]],
[[3,[7,7,[5,3,2,8,1],5,4]]],

[[[[],8,[7,7,6],1,[]],[[3,6,8,4]],[7]]],
[[4,4,1,[4]],[[0,10,9,7,10],[10,[0,6,5],[0,6,8,4]],[3,[10,6],6,[7]]]],

[[],[2,3,10],[[[1],[8,7,6,1,10],7,5],6]],
[[[[3],9],[]],[[[],[10],8,[7]],[],8,6,[0,9,[3,5,2]]]],

[[[[1,8,3,7],9,7,[],[6,3,5,2,9]],3,4,[],[9]]],
[[[[1],8,6,[]],9],[[7,[2,8,0,9]],[[4,2,5,5],5],0],[3,[[3,1,8],10,[],0,5],6,[]],[[9,[2,8,0,0,1],[],[1,1,8]],[9,9,[2,9,1,1,1],4,2],[[1],8,[0,5,6,7,8]],[7,7,[4,6,10,10],[4,0,9]],3]],

[[[[8,1,3],[5,10,7,9,5],7],3,0,7,[4,[5,4]]],[],[[[10,5,0,9],1,8,3],3,[[7],1,5,[8,3,2,10]]]],
[[[[0,7,6,5,3],[3,10,10],[2],2,5],[[],4,[],[8,7,8]],8,6,[]],[[[2]],8]],

[[],[[]],[8,2],[[4,[],0]],[[7,2,6,2,0],[[7,9,7],[1,6,1,3,1]]]],
[[],[[1,10],2,[0]],[6,3,0,3],[]],

[3,10,2,4],
[3,10,2,4,2],

[[[1,3],[[3,5,5,2]],2,3],[1],[[0,8,6,0,9],[[5,1]],[[3],[2,7,1,2,10],[7,6,3,10,5],[3,10,9]]],[3,[],[[],[],0,[9,0,10]],[[8,9,4,10],9,4,10,10]]],
[[5,7,10,1],[[[10,0,7,0,3],9],10,[[6,3,6],[6,9,9,2]],2,4],[]],

[[[[6,2,8,5,7],[5,7,7],6,[7,1,2,4,2]],[7],[2,4,[1,4,6]],[8,2,[5,4,8,1],6,[7,1,10,10]],[]]],
[[],[5,7,10],[[0,3],[[1,9,2,4],1,6,[5,2,9,8,5],8],10],[[],3]],

[[[[6,2,7,5,10],[]],8],[[9],2,[[1],5],[[4,5,7],1],[[4,0,6,0,6],9]],[9,[[5,7,5],7,[10,5,9,0,1],[5]]]],
[[[9,[],[1,6,5,1,10],[8,9]],10,8],[6,1,[8],3,0],[],[10,9,3],[6,[[6],2,[4,6,7,8,3]]]],

[[],[[5,0,3,2,8],[2,[7,10,9],[9,10,0],6],3],[3,[[10,5,7,4,7],[],6,1,10],10,[[7,5,1],6,9,[4,8,2,9,1],1]]],
[[],[[[9,2]],[4,[]],1,4,0],[[],[[0,2,6,6],1,[9,3,4,10],2],8,7,1],[2,10,1,4,8],[[[2]],[0,[]]]],

[[2,3,[0],5]],
[[[]],[],[[[2,7,2],[3,5,5],[10],[4,6,4,2],7]]],

[[[1,[0,8,2,6],[1,1,9,3,10],5],0],[[0,[],2,6,[1,5,2]],[[0,0,4,10],[5,5,7,10],[],[],[]],6],[7],[[]]],
[[],[],[0,[0,[6,5,8],7],9,3,0],[5,10],[5]],

[[2,[],[6,[7,8,6],7,6,4]],[1],[0,[8,4,3,2,[3,0,7,1,4]]]],
[[[[0,4,5]]],[[6,0,[9,8,2,4]],[],5,3,[[8],4,1]],[1,[4,1,[0,8,5]],2,[4],4],[]],

[[[],[[4,3,5,0,9],2,[],3],[],2],[[[0],3,3],4]],
[[[1,[1,7,1,2],[0,1,6,5],[5]]]],

[[0,1],[[],[7],[],[[],8,[4,8,10,9],9,[6]],5],[[],[[0,2,0,5],8,[7],5,7],9]],
[[],[7,[3,[4,5,6,8,3]]],[[[2,10,6]],6,10],[[7,[],4],[[]],10,[10,[8,5,9,0],[5]]],[4]],

[[2],[],[10,[[],[4,6,6],[8],[4,6,3,7],6],7,[]],[2,[[],[7],1]],[[[1],4,[8,7,2,7,2],9,[3,9,1,3]],9]],
[[[8,[1,2,4,3,10],[4,7,7,8,0],7]],[],[[8,[10,3,7],[8,7,6,1],8,[]],[3,5,[4],[1,1,5,4],10],9],[0,[8],[[7,5,8],[4,2,0,5,5],2,[5,10,0,0]],9],[]],

[[[[]],0,3,7,0]],
[[[8,[9,0]],2,3,[]],[[4,[7,3]],2,[4,[2,2,9,0],2],[[7,10,7],3,7,7],[[3,4,1],2,[2,1,0,2],[7,10,5,1,0],7]],[9],[],[]],

[[[1,[1,4,5],[3,10,8,8,1],[1,1,1,7],[4,5,2,2,7]],[[]],8],[8]],
[[[7,[4,3],8,3,[10,8,4]],[10,[1],2],2,8,8],[[4,4,1],[[],9,[],2,4],0],[[[6,8]],8,[4]],[5,[[9]]],[[]]],

[[],[2,3],[],[8,0,[[1,5,2,1]],9],[8,[[3,1,5,2,7],3]]],
[[],[]],

[[1,10,2,1,[9,10,[0],[9,0,9,10]]],[[[8,5],[2,1,6,2]],6],[[5],3],[]],
[[[[7]],5,[[7,5,6,1],[9,3,10],7,3],10,[4,[4],8,[8,6],[6]]]],

[[0],[[]],[]],
[[[[10,7]]],[[4,8,[0,9],[3,5,9,6],3],3,[1],[[5,7,6,3,0],[3],[],[9,2,0],[8,0]],3],[],[[9,[],[5,0],8],1]],

[[6,3,[3,[7,10,1]]],[[[6],[0],[1,7],[6,7,8,6,1]]],[8,4,5],[]],
[[3],[[7,10,2,[7,5,3,9]],[1,4],6,6,1],[],[],[1,3,[],[5,10,[],1,9],[9,8]]],

[[7],[4,10,1]],
[[1],[[],[8]],[]],

[[[]],[[[3,6,5,3],8]]],
[[[[5,1,6,0],3,1,3,7]],[[0]],[[[2,2,3],[4,5],6,4,[1]],[[9,5,6,5]],[[6,9,6,9,1]],2]],

[[7]],
[[9,[[]],1,9,[[],1,[2,0,10]]]],

[[],[1,[[7,1],[]],4],[9,3],[],[]],
[[0,[7,[],4,[8]],3],[[6,[],[7,0],[7,4,1],5],[],0,10],[10,4,5,[[1,4,2,8,6],[0,1,0,3,10]]],[],[0,[]]],

[[4],[[],10],[[[1,2,9,5,4],0,[5],[1]]],[0,0,[[5,7]],10,7],[]],
[[[8,[0,8,8,6]],6,10,5],[[],[3,[]],2,0],[2,[[8],1]],[8,2,10,[[9],[1,0]]]],

[[6,[8,[],9,[8],5]]],
[[[]],[[],3],[[[],10,3,1]],[[9,2,[],[6,9,9,2,8],2],[],6,7],[[[10,3,9]],0,[],2]],

[[7,8,8,[[3,3,1,2],0,10,[8,8,8,8,6]]],[4,[3,6,[],[1,1,6,8],9],[3,4,10,7,2]],[]],
[[[[4],4],8]],

[[6,7],[9,6,[4,[2,10],8]],[2,[[8,10],9,1],2,[[6,3,1,6],[10,4,3,9],4,[3,5,2,6,4]],[[0,5,2],10,[8,7,5,1],[4,5,4,6],3]],[],[10,0,7]],
[[7],[],[0],[6,[0,[0,10,3,4],[5,1,7,4,2],10],[2,0,[8,1,6,10],[9,2,0,9,8]]]],

[[[]],[4,[9,[0],[8,1,5,0],[10,0],[4,4,3,9,8]],[],[]],[[[8,1,9,1],[3,3,4,1],0,6],[[8],[1,9,10,6,1],[8,9,1,6],[2,2,9,0,10]]]],
[[7,6,[[0,0,2],[6,9,10,9],10],[1,10,[6,2,1],[8,7]]],[1,[[8],6,[8,8,5,8],4,[7,4,6,4,10]],[9,9]]],

[[5,[[3,4],2,8,[1],10]]],
[[[7,3,[2,8,7,5,8],8],[1,6,7,[],[2]]],[],[[0,[9],[10,4,7,1]]],[8,5],[9,9,[9,2,[7]],[2,[2,5]],[]]],

[[7],[[],[8],10,3],[4,6,[[7,8,2,6],[9,8,6,10,7],[0,7],[7,10,6]],[[10,9,1]],6],[8,[[2,8]],3],[6,0,[[2,1,2,7,6],[],7,3]]],
[[2,[],[],[[2,4,9],[3,9,5,10]]],[4,[7,5,5],0]],

[[[[5,9],2,4,5],[5,[5]],7,4,[[0,4,2,3]]],[9],[[2,0,7],4],[1,[[],[5,3,3]],[0]],[[4,7,[0,1,3,4]]]],
[],

[[2,6,6,4,1],[2,9],[3,[[5],5],4,[],2],[9],[7,9]],
[[4,1,[]],[],[[[6,0,1]],0,[[7,4],10,2],[[4,8,2,6,6],3]]],

[[[[6,4,2,2,9],[5],[6,10]]],[6,[4,[0,1,7,3]]],[],[[[7],[9,10]],[1]]],
[[[[8,10],9,[4,9],9],[[9,3,10,10],[6,5,2,8,7],[],[],8],[[],2,9],10],[[1,3,6],[1,0,[5,4,9,4],0,[0,6,4,5,4]],5],[0],[1,6],[4,[[2,8],[2,1,9],8,[8,5],[4]],3,3,[7,[2,10,2,1,10]]]],

[[1,8,7,6],[[9,[6]],[1,[5,2,6],[],[7,7,0]]]],
[[7],[[6,8,6,[10,10],2],10,3,[]],[],[1,[[],[3,10,6,4,10],4,5,4],[6],2,3]],

[[9,[8,8,9,[8]],[4,[8,1,4,8],[6,9],3],5,[8,[9,7,2,2]]],[0,0,[5],2,[1,[2,8,5,4,4]]],[9]],
[[[[2,5,10,2,10],4,6],[[2]],8,10,[8]]],

[[[[7,9,7,1],[6,9,10,9]],[3,[]],0]],
[[[8,[6,5,4,3]],[[1]],[1,[5,0,6,3,3]],1],[[1],3,8]],

[[[[10,10,7],[8,10,9,1],[9,8],1,9]],[[[8,8,6,0]],3,3,[9],6],[1,8,[4,[9]],9,[[9,0,3],[4,10],[0,2,8,2],3]]],
[[[],[0,1,[]],0,[[3],9,3,10]]],

[[[7,10,[1],2,[6]],[6,[0],3,[],6],[[4,4,9],2]],[[[],0,[3],[1,6,10],[6]]],[]],
[[10,[],[7,3,8,[1,6,6,2],1],9],[[[5],[8,5],6,9],[[],4,8,8,6],10],[],[[[2,5,7,6,0],[8,9,1,8],[],8],[2],[]],[[[9,9,2],[10,5,9],[0,4,5,2],[]],3,[[8,2,8,8,3]],[[8,2,0,8,2],[5,5,3,10,6],[2,4],[]],[[9,2],8,[9,8,10,6,2],3,[10,5,3,7,3]]]],

[[[],7]],
[[[],[],0,10],[[[],8],9,[[8,5,7],[3,7],10,[6,9,4],1],[[4,4]],7],[],[9,4,[[6,3,6],6],9,[[9,5,7,3,8]]]],

[[[7,[3,10,7]],5,3]],
[[],[6,[6]]],

[[1,0]],
[[0,1,[[7,8,7,4],7,[],10],[10,5,0,7,1],[[9],9,[9,6,10],7,4]],[10,[9,[1,3,5,7],8,4]],[[[8,2,5],8,[6],[],[]],0,6,9],[[10,[],[6,10,2,8,1],[2]],7,2],[[2,[5],7,1,[]],[[1,2,3],1,[6,7,6,2,8],[2,8,2,6,1]],6,10]],

[[0,[8,[10,4,0,10,6],10,5,0],[[]],6,4],[],[],[3,3]],
[[],[[1,3,4],[10,4,10],2,[1,8,5,3],[[5,1,8,10,5],10,[1,2,3,0,2],[10,6],[3,2,2]]],[[3,7,[5,5,0,10],[0,7]],3,2]],

[[[3,[6,3,2]],[[6,5,10,10,9]],[[0,6,9,8],[],[10,6,5,8],6,10],3]],
[[9,7,[6],4,4],[],[[1,0,3,[10,6,7],[]],[10,[6,0,3,4],4,2,[1,0,2]],4]],

[[[[10,2],[],[],[0,10,6,10]]],[[[7,10,8],6,[10],[9],[4,4,2,2,9]],[1,6,6,10],5,2],[8,10],[[[],7,7,[10,1,5,10,10]]]],
[[[3,6,[4,10,2,6,8],[0,2,3]],2,6,8,10],[0],[5,[[1,6,4,0]],2],[10,[],6,9,[3]]],

[[0,9,[[9,2,0,1],6]],[],[1,[[9,7,6,0],[8,4,3,7],0]],[[10,[0,2,5],[7,4],[7,2,1,9,9],[2,9]]],[[8,[1]],[]]],
[[5,2,1],[],[[0,9,[1,1],0,2],0,[10,[9,0,1,3,9],[9]],4],[8,7]],

[[[1,[9,10,9],7,9],[[7,9,0,1],7]],[[[8,7,9,2,3]],[[]]]],
[[[[]]]],

[[[[5,0,10,6],[7,2,9]],0,[[],[10]],[8,[7,0],7,[10,4,7]],[]]],
[[3,[10,2,[1,10]],1,3,[[0,10,8,9,7],[8],[6,6,3],10]],[],[1,[[9,5]]],[9,8,[10,9],[[0,2,0,9,6],[9,0,1],5]],[[[0,7,9,9,7],[8,8,5,6],8,[4,1,4,2,4],[9,3,4,5,9]]]],

[[[9,[]],9,[4,[],[4]]],[],[[1,[7,6,4,2],[5,10,3,5]],[1]],[[[3],[9],8,[7,4,5,8],2],0,[1,7,7,[8,6],1],[]],[5]],
[[2,9,6]],

[[9],[],[[7,7,4,[0],9],3,[[6,10,8],4,6,0]]],
[[[[9,4,4,8,3],[5,6,4],[]],[[9,2,2],[4]],9],[],[0,10,[[10,9,10,10],[4,3]]],[7,7,[[9],[5,9,0],[6,7,10,2],[1,4],9],[9,[1,7,4],8,[10,7],[0]],[]],[[[1,1],10,[0,7]],[2,[10],7,9,2],2,10,6]],

[[2,[1],2,4,[3,4]],[3,2],[0,[[0,10,7],[5,0,5,4],7,[5,4,7]],[[2,4,3],[8,5,3,5,1],4,10]],[8],[7,[5,2]]],
[[2,6],[],[0,7,[9,[8,6]],2,2],[]],

[[[[6,10,5,5],9],6,[[5,2,6,2],9,6,[9,10,6,1,7],4]]],
[[9,[10],0],[[1]]],

[[],[10,[5]],[4,[[],8,[5,6,8,4,4]],[[5],6,5]],[[[],7]]],
[[],[[6,[0],[5,8,0,10]],5,[10],4],[[[10],1],[8,2,6,[0,3,6,1],8],3],[]],

[[[],2,[7,[9,10]],[]],[],[3,5,[[3,3],6,[]],2]],
[[7,[[],3,0],3,8,6],[],[[2,7,10,[6,3,6,6],7],[[]]],[5]],

[[],[[],0,[],1],[],[]],
[[],[[]],[[2,[]],[[],2]],[]],

[[4,[7],6],[5,0,[[10,6,5,10,0],1,[10,1,5,10]],1],[0,[],7,5],[2,2,[2,1]]],
[[5,[2]],[[[0,7,4]],[],6],[9,0],[[],[0],2,[1,9,[7]],0]],

[[],[[[9,5]]],[[8,2,[2]],8],[[[5],7,[9,4,4,0,10]],0,[[6],[2,4,0,7,9],[],[1,1,7]],3]],
[[[]],[[10,2,[1,1,4,9]],0,[10,[6],9],6,[8,8,4,6]],[1,7,7],[],[0,10,[],6]],

[[[[9,6,7,7],7,[1,1,9],6],5,3],[2,[],[5,[2,2,2,7,3],3],3],[3],[4,2,[[9,1,2],7],3],[[[1,1,0,6],[8,5,10,4,2],1,0,[10,9,6]],[5,5,4,9],[[5,2,5],[3,10],[0],[10,5,2,4]],[9,[2,2,5],6,1,2]]],
[[],[[[5],9]]],

[[[3,[6,2],1],[[9,1],[],4],1],[[[8],[]],[],9,[]],[6,6,[10,0],10]],
[[3,1,[[2],6,[],[4]]],[[5,4],[6]],[4,2,8,0,[]],[4,[[8,9,1],[3,3,8,4,6],1,2],5,10,[[3,10,10,8],4]],[]],

[[4,1],[],[]],
[[6,3,[4,0,[2,7]]],[[8],[1,[9],8,10],[7,[9,4,6],3]],[[4,1],[3,9,7,9],[],[10,8,6,1]],[6,5,[[8,10,9,5,8],5,0,[9,1,9,5],1],8],[2]],

[[[5],7,[5,1,10,10,[1,4]]],[[[5,4],[8,5]],1,[5],[],[8,[3]]],[[7,6,3,[10],9],4,6],[[],5]],
[[[2,4,[7]],[[1]],[8,9],[[0,1,3,6],[7,8,0,2],[],[8]]],[],[6,0,[],[]],[2,[],0],[6,[0,1]]],

[[4,[],0],[],[10,5,[]],[3,4,[[6,6,6,6],[9,10,6,4,0],5],7,[]]],
[[8,1,3],[[5,0],0],[3,[[3],8,[1,8,0,5],7,4],[4,[1,9,9],[],7],[[8,9,3,0]]]],

[[[1,2,[3,1,0,8,0],4],[1,[10,0,6]],8,[[7,2,0,8,4]],[[5,10,5],9,1,[]]]],
[[[[],[],[10,8,3,3,5]],[9,9,[7,6,6,2]]]],

[[[],[[10,5]],6,[0,6,[9,0,1,8],8,7],7],[[[2],[6,10],4,[5,0,6,6,1],10],9,7,[],8],[],[[],[[9,9,3],6,[2,5,5],[8]],10]],
[[[[2,5,8,5,5],1,7,7,[2]]]],

[[[7,[0,6],9,[8,8,3],[4,10,7,7,9]],[[8],[],[0,0,10]],[],[]],[[[4,2,1,5,5],7,[2,3],7,3]],[10,[[10,4,7,5,6]],1]],
[[5,[],5],[3,0],[0],[7]],

[[5,[10,[7,10,6]],[[6,5,5,8],[5,2],4,10]],[],[2,[[],0,[6],3],5,6,[3,10,5,[2]]],[7,10,8,[[10,6,8,6],10,[],[7],[10]]],[]],
[[8,10,6]],

[[0,[]]],
[[]],

[[[6,[8,6,9],8]]],
[[],[9]],

[[10],[6,[[2,6,9,5],[0,4,2]]],[[[]],3,8,[],[[0,8,5,0],6]],[4,[],3,7]],
[[[],6],[[10],[[7,8,1,8,2],[],[5,10,4],8,2],[[10,9,9,0,3]]],[3,[5,[7]],[[10,1,10,10,0],3,3,1],[1,6,[0],0,[5]]],[[8,9,[0,0,10,8]],[1]],[6,7]],

[[6],[3],[]],
[[[[2,5,3],9],[[5,10],8],5,[7,7,[],[2]]],[[7,[4],[4],5,0]],[10],[[10,3],[[],4,8,2,[10,3,6]],7]],

[[1,[0,4,6]],[[],7,[8,5],[[4,7,4],7,[3],1,9],5],[[9,[9],3,[8],[4,4,8]],6,10],[[0,[7]],[[],8],6],[4,[[1],8,[6,4,2]],6,[2,3,[1,7]],4]],
[[8,[],10]],

[[[6,2,[7,9],10,7],[5,[1]]]],
[[[8,6,[3]],6,7,[[7,10,4],[6,1,3,9,5],[0],5]],[5,9],[8,4]],

[[2,2]],
[[[[9,5,8],[10],1,[3,3,8,8,6],[1]],9,3,[[6,9,6,9],9]],[],[6,[[8,3],[4],7,7,9],8,5,[7,9,7,[8,5,9,0,8]]],[[7,[8,10,3,7],0,[1,3],4],7]],

[[[2,5,[9],[],8],10,2,2,0],[],[]],
[[9]],

[[[3,4,[4,4,5],[10]]],[]],
[[],[[[6,10,7,5,6],[1,10,9]],3,[[2,2,3,10,0],[]],[[8,10,8,6],[1,3,4,5],2]],[1],[]],

[[[8]],[[],0,10,9],[[5,[10],6,6,6],[[5,8],[1,3,4],7,[4,1,8,2,9]],4],[3,[[],[],[0,4,6,1,0]],6,[8,[7,5],0,0,[]],4]],
[[[3,5,0,[2,4,8]],[4]]],

[[4,[8,9,3],6,[7,[0,0,2,8,3],[4,5],[5],[10,4,3]]],[2,1,[6,6,6,[3,1],6],[6,0,10,[1,3,8,8]]]],
[[[],3],[[[1,7,6,7],8,[],0,5],[4,8,[]]],[1,5,[[2,9,10,3],[6,6,1,10],0]]],

[[[6,4,7,7],4,6],[4,[2,1,9,[8,2,2,0]],8,[[],[9],[4,10,4,9],[0,5],[10,8,3,3,1]],[]],[[7],2],[6,[[8],[]]]],
[[[[7,5,8]],[1,10,[8],[2,8],[2]]],[7],[[[2,1,10,1,7]],[],3]],

[[[],[5,[1,5,10,5,1],9,[8,4,1,6,9],[7,4,6,7]]],[[9,6,[2,10,3],3],10,6],[2,7,[3,1,[10,5]],[]]],
[[3,10,1,[3,[],[3,4,4,10],0],2],[[[2,10,0],[5,1,0,5],[3,10,9,4,9]],[10,4,[10,6,5,2,9]],3,7,6],[3,[[4,5,6,1,1],[3,2,9,0,5],[4,5,9,4],5,0],[[0,9],8,[6,2],9,0],8],[3,[[7,8],5,10]],[1,[2,0,8,[3,7,4,10],7]]],

[[7,[],8,9],[[[5],[]]],[[7],1,[6,[6,7,10,5,6],6],0],[[[1]],[[10,4],[],8,0,[0,4,0,0,1]],[[],[1,5,10,2,4],[8,2,7],[]],[3,[8,1],2,[0,6,3],7],0],[[8,[5,5,8,4],[10,5,6,6],[8,0,8],4],10]],
[[[[5,3],7,[],2],[1,10,[7],5]],[[0,[2],[3]]],[4,9,[[]],9]],

[[0,[],[7,[7,4,9,5]],3,9],[[[2,9,3,10],[3]]],[[],6,1,2,7],[1],[]],
[[0,[4,[8,3,5]]],[[[1,4,5,0],9,5],[[7,2,3,7]],5],[0,1],[],[6,6,7,2]],

[[9,9,[[10],9,[],[1,10],[10,3,8,10,3]],[10,7,5],[]],[4,5,4,3],[10,[],6],[[3,8,[6,8],[3]],0,2,1],[[5,0,[9],10]]],
[[8,[[],[4,10,0,2,9],6,[],[8,0]]],[1],[[[4,9,0],[7,2],0,9]],[],[[7],[1],9,[0,[],9,9,9],[[1],3]]],

[[10,[],0],[[],[],[[1,2,6],8,[1,2,6,2],[3,4,6,5,4],[2,3,7,8,1]],[[7,3],0,6,[0,2,8,2],4],[[],5]]],
[[[10,[7,3,8,3,5]],8],[10,[[4,10,2,6],7,[6,9,10,0,9],10]],[[],2],[10,1,6,6]],

[[9,[]],[[],[9,[9,5],0,[0],[9,0,5,8]],6],[6]],
[[[[7],8,7],5,[[7,4,1,2],2],[2,6]]],

            [[6]],
            [[2]]
        ]"#;

    let v: Value = serde_json::from_str(data)?;
    let parsed = v.as_array().unwrap();

    let mut i = 1u32;
    let mut sum = 0;
    for pair in parsed.chunks(2) {
        if compare(&pair[0], &pair[1]) == Ordering::Less {
            sum += i;
        }

        i += 1;
    }

    println!("Part 1: {}", sum);

    let mut sortable = parsed.clone();

    sortable.sort_by(compare);

    let mut key = 1;

    for (i, e) in sortable.iter().enumerate() {
        if !e.is_array() {
            continue;
        }

        if e.as_array().unwrap().len() != 1 {
            continue;
        }

        match e.get(0) {
            None => {}
            Some(v) => {
                if !v.is_array() {
                    continue;
                }

                if v.as_array().unwrap().len() != 1 {
                    continue;
                }

                match v.get(0) {
                    None => {}
                    Some(v2) => {
                        if v2.is_number() && [2, 6].contains(&v2.as_u64().unwrap()) {
                            key *= i + 1;
                        }
                    }
                }
            }
        }
    }

    println!("Part 2: {}", key);

    Ok(())
}

fn compare(a: &Value, b: &Value) -> Ordering {
    // println!("Compare {} vs {}", a, b);
    if a.is_number() && b.is_number() {
        let a_n = a.as_u64().unwrap();
        let b_n = b.as_u64().unwrap();

        match a_n.cmp(&b_n) {
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => {}
            Ordering::Greater => return Ordering::Greater,
        }
    } else if a.is_array() && b.is_array() {
        let a_a = a.as_array().unwrap();
        let b_a = b.as_array().unwrap();

        let max_cmps = [a_a.len(), b_a.len()].iter().min().unwrap().to_owned();

        for i in 0..max_cmps {
            let result = compare(&a_a[i], &b_a[i]);
            if result != Ordering::Equal {
                return result;
            }
        }

        match a_a.len().cmp(&b_a.len()) {
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => {}
            Ordering::Greater => return Ordering::Greater,
        }
    } else if a.is_array() {
        return compare(a, &Value::Array(vec![b.clone()]));
    } else if b.is_array() {
        return compare(&Value::Array(vec![a.clone()]), b);
    }

    Ordering::Equal
}

#[cfg(test)]
mod tests {
    use crate::compare;
    use serde_json::Value;
    use std::cmp::Ordering;

    #[test]
    fn comparisons() {
        let data = r#"
        [
            [1,1,3,1,1],
            [1,1,5,1,1],

            [[1],[2,3,4]],
            [[1],4],

            [9],
            [[8,7,6]],

            [[4,4],4,4],
            [[4,4],4,4,4],

            [7,7,7,7],
            [7,7,7],

            [],
            [3],

            [[[]]],
            [[]],

            [1,[2,[3,[4,[5,6,7]]]],8,9],
            [1,[2,[3,[4,[5,6,0]]]],8,9]
        ]"#;

        let v: Value = serde_json::from_str(data).unwrap();
        let parsed = v.as_array().unwrap();

        let result = compare(&parsed[0], &parsed[1]);
        assert_eq!(result, Ordering::Less);

        let result = compare(&parsed[2], &parsed[3]);
        assert_eq!(result, Ordering::Less);

        let result = compare(&parsed[4], &parsed[5]);
        assert_eq!(result, Ordering::Greater);

        let result = compare(&parsed[6], &parsed[7]);
        assert_eq!(result, Ordering::Less);

        let result = compare(&parsed[8], &parsed[9]);
        assert_eq!(result, Ordering::Greater);

        let result = compare(&parsed[10], &parsed[11]);
        assert_eq!(result, Ordering::Less);

        let result = compare(&parsed[12], &parsed[13]);
        assert_eq!(result, Ordering::Greater);

        let result = compare(&parsed[14], &parsed[15]);
        assert_eq!(result, Ordering::Greater);
    }
}

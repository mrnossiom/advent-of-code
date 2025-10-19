let content = "";

const MAX_PER_COLOR = { red: 12, green: 13, blue: 14 };

let sum1 = content
  .split("\n")
  .map(line => line.replace(/^Game /, ''))
  .map(line => line.split(": "))
  .map(([strId, games]) => {
    let foo = games
      .split("; ")
      .map(game => game
        .split(", ")
        .map(info => {
          let [nb, color] = info.split(" ");
          return parseInt(nb, 10) <= MAX_PER_COLOR[color];
        })
        .reduce((acc, cur) => acc && cur, true)
      )
      .reduce((acc, cur) => acc && cur, true)
      ;
    return [parseInt(strId, 10), foo];
  })
  .reduce((acc, [id, foo]) => foo ? acc + id : acc, 0)
  ;


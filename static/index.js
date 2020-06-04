let choice = null

const chose_black = () => {
    let placeholder = document.getElementById("choice_placeholder");
    choice = "black";
    placeholder.innerHTML = choice;
    console.log(choice);
}

const chose_white = () => {
    let placeholder = document.getElementById("choice_placeholder");
    choice = "white";
    placeholder.text = choice;
    console.log(choice);
}

const find_pair = () => {
    let result_placeholder = document.getElementById("result");
}

const main = () => {
    document.getElementById("white")
        .addEventListener("click", chose_white);
    document.getElementById("black")
        .addEventListener("click", chose_black)
    document.getElementById("find")
        .addEventListener("click", find_pair)
}

main()

import markdownit from 'markdown-it';
import { container } from "@mdit/plugin-container";
import { mark } from "@mdit/plugin-mark";
import * as content from 'bundle-text:./story/**/*.md';

function escapeHTML(str){
  return new Option(str).innerHTML;
}

const md = markdownit().use(container, {
  name: 'sms',
  openRender: (tokens, index) => {
    const token = tokens[index].info.trim().slice(4).trim().split(' ');

    let me = false;
    let image = '';
    let name = '';

    for (let word of token) {
      if (word === '') {
        continue;
      } else if (word === 'me') {
        me = true;
      } else {
        let [key, value] = word.split(':');

        if (key === 'name') {
          name= value;
        } else if (key === 'image') {
          image = value;
        } else {
          throw new Error("unexpected key "+key);
        }
      }
    }

    return `
      <div class="sms ${me ? 'me': ''}">
        <img class="sms-photo" src="${escapeHTML('/'+image+'.svg')}">
        <div>
          <div class="sms-author">${escapeHTML(name)}</div>
          <div class="sms-content">
    ` 
  },
  closeRender: (tokens) => {
    return '</div></div></div>\n'
  }
}).use(
  mark
);

const unindent = ([s]) => {
  lines = s
    .replace(/^\n|\n *$/g, "")
    .split("\n")
    .map((i) => i.trimEnd());
  let min_spaces = lines
    .filter((k) => k.trim() != "")
    .map((i) => i.length - i.trimStart().length)
    .reduce((a, b) => Math.min(a, b));

  return lines
    .map((i) => i.substr(min_spaces))
    .reduce(
      (a, b) =>
        b === ""
          ? a + "\n\n"
          : a.endsWith("\n") || a === ""
          ? a + b
          : a + " " + b,
      ""
    );
};

const documents = {
  briefing: content.briefing,
  interviews: {
    "Rufus Red": content.interviews.rufus_red,
    "Dianna Robinson": content.interviews.dianna_robinson,
    "Judy Woolridge": content.interviews.judy_woolridge,
    "Duncan Moss": content.interviews.duncan_moss,
  },
  "victim's laptop": {
    email: {
      "3-1-2024": content.laptop.emails['3-1-2024'],
      "7-1-2024": content.laptop.emails['7-1-2024'],
      "12-1-2024": content.laptop.emails['12-1-2024'],
    },
    messages: {
      D14nna: content.laptop.messages.dianna,
      Duncan: content.laptop.messages.duncan,
      BikerBro: content.laptop.messages.rufus,
      JuStar: content.laptop.messages.judy,
    },
  },
  questions: {
    type: "questions",
    questions: [
      {
        name: "Who did it?",
        options: [
          "Rebecca",
          "Rufus",
          "Duncan",
          "Dianna",
          "Judy",
          "Greyham",
          "Jace",
          "David",
          "Other",
        ],
        answer: "Judy",
      },
      {
        name: "What emotion motivated the crime?",
        options: ["Love", "Hate", "Greed", "Fear", "Other"],
        answer: "Love",
      },
      {
        name: "Who would benefit the most from Rebeccas death?",
        options: [
          "Rebecca",
          "Rufus",
          "Duncan",
          "Dianna",
          "Judy",
          "Greyham",
          "Jace",
          "David",
          "Other",
        ],
        answer: "Rufus",
      },
    ],
  },
};

const startGame = () => {
  let state = [documents];

  const root = document.getElementById("root");

  // UI Manipulation Code
  const createMenu = (new_state, index) => {
    const div = document.createElement("div");
    div.classList.add("menu");

    for (const key of Object.keys(new_state)) {
      const element = document.createElement("div");
      element.classList.add("menu-item");
      element.dataset.name = key;

      const content = document.createElement('div');
      content.classList.add('menu-item-content');
      content.textContent = key;

      element.appendChild(content);

      if (key == "questions") {
        element.classList.add("menu-item-questions");
      } else if (typeof new_state[key] == "string") {
        element.classList.add("menu-item-evidence");
      } else {
        element.classList.add("menu-item-folder");
      }

      element.addEventListener("click", () => {
        updateUIColumn(index, key);
        updatePath(index, key);
      });

      div.appendChild(element);
    }

    root.appendChild(div);
  };

  const createEvidenceContainer = (root) => {
    const div = document.createElement("div");
    div.classList.add("evidence");

    const div_outer = document.createElement("div");
    div_outer.appendChild(div);
    div_outer.classList.add("evidence-container");

    root.appendChild(div_outer);

    return div;
  };

  const createQuestions = (new_state, index) => {
    const div = createEvidenceContainer(root);

    const questionAnswers = [];

    for (const question of new_state.questions) {
      let id = question.name.replace(/[^a-z]+/g, "-");

      const label = document.createElement("label");
      label.for = id;
      label.textContent = question.name;
      div.appendChild(label);

      const br = document.createElement("br");
      div.appendChild(br);

      const select = document.createElement("select");
      select.id = id;

      for (const optionText of question.options) {
        const option = document.createElement("option");
        option.textContent = optionText;
        option.value = optionText.replace(/[^a-z]+/g, "-");

        select.appendChild(option);
      }
      div.appendChild(select);

      const br2 = document.createElement("br");
      div.appendChild(br2);

      questionAnswers.push([select, question.answer.replace(/[^a-z]+/g, "-")]);
    }

    const verifyButton = document.createElement("button");
    verifyButton.textContent = "verify";

    const verify = () => {
      div
        .querySelectorAll(".error,.success")
        .forEach((el) => div.removeChild(el));

      let errors = 0;
      let total = 0;

      for (const [select, answer] of questionAnswers) {
        total += 1;
        if (select.value != answer) {
          errors += 1;
          let error = document.createElement("div");
          error.classList.add("error");
          error.textContent = "incorrect";
          select.parentElement.insertBefore(error, select);
        }
      }

      if (errors > 0) {
        let error = document.createElement("div");
        error.classList.add("error");
        error.textContent = `${errors}/${total} answered correctly`;
        verifyButton.parentElement.insertBefore(error, verifyButton);
      } else {
        let error = document.createElement("div");
        error.classList.add("success");
        error.textContent = `All questions answered correctly`;
        verifyButton.parentElement.insertBefore(error, verifyButton);
      }
    };

    verifyButton.addEventListener("click", verify);

    div.appendChild(verifyButton);
  };

  const updateUIColumn = (index, name) => {
    console.log(index, name);
    root.childNodes[index].childNodes.forEach((element) => {
      element.classList.toggle("selected", element.dataset.name === name);
    });
    while (root.childNodes[index].nextSibling) {
      root.removeChild(root.childNodes[index].nextSibling);
    }

    state.splice(index + 1, state.length - index - 1);
    let new_state = state[state.length - 1][name];
    state.push(new_state);

    if (name === "questions") {
      createQuestions(new_state, index + 1);
    } else if (typeof new_state === "string") {
      const div = createEvidenceContainer(root);
      const text_html = md.render(new_state)
      div.innerHTML = text_html;
    } else {
      createMenu(new_state, index + 1);
    }
  };

  // URL manipulation code
  let lastPathName = "/";

  const updatePath = (index, name) => {
    const path = window.location.pathname.substr(1).split("/");
    path.splice(index, path.length - index);
    path.push(name);
    lastPathName = "/" + path.join("/");
    window.history.pushState(undefined, undefined, lastPathName);
  };

  const onPathUpdate = () => {
    const path = window.location.pathname
      .substr(1)
      .split("/")
      .map(decodeURIComponent);
    const lastPath = lastPathName.substr(1).split("/").map(decodeURIComponent);
    for (let i = 0; i < path.length; i++) {
      if (path[i] != lastPath[i]) {
        updateUIColumn(i, path[i]);
      }
    }
    lastPathName = "/" + path.join("/");
  };

  window.addEventListener("popstate", (ev) => {
    onPathUpdate();
  });

  // Startup code
  createMenu(documents, 0);
  onPathUpdate();
};

startGame();

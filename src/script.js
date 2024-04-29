import markdownit from 'markdown-it';
import { container } from "@mdit/plugin-container";

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
});

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
  briefing: unindent`
    At 15:14, An ambulance where called to the address 412 Holy Ave. There they found
    REBECCA RED, crushed by a falling closet, while moving into her new house. She did not survive.

    There where others 4 people present:

    - Rufus Red

    - Duncan Moss

    - Dianna Robinson

    - Judy Woolridge

    Was this a tragic accident or foul play?
  `,
  interviews: {
    "Rufus Red": unindent`
        Greenfield Police Department

        ----------------------------

        Interview started at Sunday 14-1-2024 17:26

        **Officer:** How do you know Rebecca?

        **Red:** She is my sister.

        **Officer:** You live all the way in Boxon right? When did you arrive in Greenfield?

        **Red**: We had some family stuff me and Red needed to take care of. Because of his new job Red
         was unable to come to Boxon because of his new job, so me and Yellow came here last week to arrange stuff, also to congratulate him on his new job and house.
 
        **Officer:** Can you recount the events of this morning?

        **Red:** Nothing event full really happened all morning. I arrived at Rebecca's
        old flat at around 9 AM. We loaded the truck during the morning. 

        **Red:** After unloading the truck onto the pavement, Around 1 PM Judy went to bring it back to the rental
        place. I was assembling some shelves in the basement. I heard the sound of Judy coming back, Some time later
        I walked up, there I found everyone else just standing in a circle outside, silent, not saying anything.

        **Red:** I think we where all in shock. Then Duncan arrived, not from the house, IDK where he was. He spanned us out
        of it and called an ambulance. We didn't have much hope.

    `,
    "Dianna Robinson": unindent`
        Greenfield Police Department

        ----------------------------

        Interview started at Sunday 14-1-2024 17:43

        **Officer:** How do you know Rebecca?

        **Robinson:** We have been friends since college. Did every project together

        **Officer:** Can you recount what happened exactly on Sunday?

        **Robinson:** I was here around 10:30, I live just around the corner. Red and his family where planning to
        load the truck and I was waiting at the destination to help unload. I live nearby.

        **Robinson:**I had to wait till nearly 11 before Red arrived to let me in. After we unloaded the truck, I was just unpacking
        unpacking some equipment for what would be the office area for the startup. 
    `,
    "Judy Woolridge": unindent`
        Greenfield Police Department

        ----------------------------

        Interview started at Sunday 14-1-2024 18:03

        **Officer:** How do you know Rebecca?

        **Woolridge:** We go way back to elementary school, so we've been friends for
        at least 18 years, and since I'm only 23 that's a long time.

        **Officer:** It says here you come from Boxon, it's a long way away

        **Woolridge:** Duncan can't drive, so he needed someone to bring him here.
        Won't say no to a chance to visit the city. I'm not doing much with my
        life right now so if an opportunity to help someone out comes along I'm
        always happy to take it.

        **Officer:** Can you recount exactly what happened today?

        **Woolridge:** Well nothing really special happened, we loaded the truck, unloaded the truck, I brought the truck back to the shop,
        then when I came back I found bits of wood scattered across the whole lawn and right in the middle of it the wreck
        of a large closet. Nobody was around, I found Rufus and he was the one that noticed the blood.
    `,
    "Duncan Moss": unindent`
        Greenfield Police Department

        ----------------------------

        Interview started at Sunday 14-1-2024 17:04

        **Officer:** How do you know Rebecca?

        **Moss:** We had been dating for 6 months

        **Officer:** Can you recount what happened today?

        **Moss:** Everyone was acting super odd from the start. They where all still mad at each other from last night I guess
        First Judy and Rufus arrived together. Judy was sleeping on the couch in the old appartment the night before
        but didn't show up Saturday night, I guess they where together.

        **Moss:** We loaded the truck mostly in silence, drove to the new place. Dianna was already there. We barely unloaded the
        truck before Judy drove off in it. Guess the whole situation was too uncomfortable for her too. Dianna and Rufus
        seemed to purposefully avoid each other, carefully looking if the other wasn't there before coming to the entrance
        and picking up stuff.

        **Moss:** I realized we didn't have any soap or toilet paper yet, so I walked to the store to get some. It's just a few
        minutes away. When I came back I found her. Everyone was right there not doing anything, like they didn't even care.
        I called an ambulance. 

        **Officer:** What happened Saturday night that made everyone so mad?

        **Moss:** So emotions where tense from the very start. Which makes sense considering the grief. Seemed Rufus felt
        Rebecca didn't care enough, but she just shows her emotions in a different way. Surely her brother would know that?

        **Moss:** I don't know what exactly happened Saturday. We where all going to get dinner together at that indian place,
        I was a bit late. I work part time on Saturdays. Something had happened. Rufus and Judy where yelling at Rebecca and Dianna. Dianna
        was also yelling. Made quite a scene.

        **Moss:** Judy announced she and Rufus where leaving and basically dragged him with her. Dianna also left.
        Just me and Rebecca where left and we had dinner with just the
        two of us.

        **Moss:** Rebecca refused to tell me what it was about. She was clearly shaken though. She said she wondered if she did
        the right thing but wouldn't tell me what the thing was.

        **Moss:** One of them is responsible for sure. They let their emotions get the better of them.
        `,
  },
  "victim's laptop": {
    email: {
      "3-1-2024": unindent`
              from: academic-integriy-office@greenfielduniversity.edu
  
              to: rebecca@email
  
              subject: Official Warning regarding plagiarism
  
              -------------------------
  
              It has come to our attention that your groups final report for the subject "2IK30
              Advanced Organic Chemistry" contained sections copied verbatim from online sources.
  
              In accordance with our academic integrity policy, your score for the subject has
              been voided, and you receive a official warning. A second instance of misconduct will
              result in immediate expulsion.
  
              If you believe there has been a mistake, please fill in the "Academic Warning Objection
              Form" and send it to the study board. Note that less than 1% of objections lead to
              the revocation of a strike.
          `,
      "7-1-2024": unindent`
            from: david@abcd-angel-investors.eu

            to: rebecca@email

            date: 10-1-2024

            subject: Hiring your starting staff

            -------------------------

            Hi Ms. Red,

            In our last meeting we discussed the process of hiring your companies initial staff.
            
            Has Susan gotten back to you yet? I reviewed her resume and she seems a good fit. Consider
            offering het more compensation.

            Glad to hear Harry and Hugh accepted their offers. Should be enough to get started actually
            building the product next month.
            
            Regarding your request to make an exception to our hiring policy for your friend, I spoke
            to our investment coordinator and I'm afraid we can't do that. When attracting investors,
            especially as a young person, credentials are everything. Thus we will require all staff
            to have masters degrees until your debt is paid off. I'm sorry. Be careful making promises
            in the future until the entire situation is known.

            About the planning you submitted: In my opinion, it's a bit too optimistic. Even if 8 weeks
            would normally be enough time to build a prototype, there will be a lot of admin work to
            do during this time to get the new company set up. Estimating about half velocity for the
            first six months is very reasonable, not to mention it would require a whole team and you
            have found just 2 people so far.
            
            Stressing to get a working prototype for TMSC seems like a waste of time. Creating something
            that may not work properly in 8 weeks may hurt your chances in the future. The TMSC conference
            is mostly targeted towards the industry while right now you need to prioritize finding more
            investors.

            I'd suggest aiming for a first prototype for ACL in April.

            Also congratulations on finding the new place. Having a fixed location should really help
            making the company feel real.

            - David Brooke

            Senior Startup Coach,

            ABCD Angel Investments
        `,
      "12-1-2024": unindent`
            from: jace@lawyers.email

            to: rebecca@email

            date: 12-1-2024

            subject: Re:Last Will of Alexander Red

            -------------------------

            In regards to your questions:

            Your father's Company "Red's Bikes" was valued at ¤1,542,000 during the december audit. You may object
            to this valuation, but I doubt it will change much. When an asset is left to multiple beneficiaries, you
            have the following options:

            1. Each beneficiary pays ¤1,542 in inheritance taxes, and gain joint ownership of Red's bikes. There
            may be more costs involved in changing the registration status of the company, creating a co-ownership
            contract, and other related fees.

            2. One or more beneficiaries can buy out each others shares. Since there are two in this case, either one
            of you could pay ¤771,000 to the other plus ¤3,084 in inheritance takes, and take complete control
            over the company. Note if you agree to pay a different amount you may need to pay additional gift tax
            on the difference.

            3. If neither party wants to buy out the shares and the parties are unable to come to another agreement,
            the company will be sold on the public market, overseen by the executor (which is me). The proceeds, minus
            inheritance tax and other fees, will be divided over the beneficiaries. 

            You have until 31-1-2024 to reach an agreement.

            Let me know if you have any more questions.

            Jace Woods,

            Estate Lawyer
        `,
    },
    messages: {
      D14nna: unindent`
            ### 19/12/2023

            ::: sms name:Rebecca image:rebecca_red me

            So I was talking to the guy from ABCD investments, they have approved our funding. Seems
            our little toy project is going to become a reality.

            :::

            
            ::: sms name:Dianna image:dianna_robinson

            How that's amazing. What's the fine print though?

            :::

            
            ::: sms name:Rebecca image:rebecca_red me

            They have a bunch of rules the company has to follow, stuff like keeping the accounting in
            order and maintain reasonable professionalism.

            :::

            ::: sms name:Dianna image:dianna_robinson
            
            That's fantastic. 

            :::

            ::: sms name:Dianna image:dianna_robinson

            BTW did you finish your part for the advanced chemistry paper?

            :::

            ### 20/12/2023

            ::: sms name:Dianna image:dianna_robinson

            Did you turn in the final project?

            :::

            ::: sms name:Rebecca image:rebecca_red me

            Oops nearly forgot. Ouch I see some sections are not filled in yet.

            :::

            
            ::: sms name:Dianna image:dianna_robinson

            Those where yours right? I know you are got some extra credits but I need to pass this or I'm
            not graduating next week.

            :::

            ::: sms name:Rebecca image:rebecca_red me

            Sorry, I'll try to fix it. Deadline is 12 right?

            :::

            ::: sms name:Dianna image:dianna_robinson

            It's 11

            :::

            ::: sms name:Rebecca image:rebecca_red me

            Ah shit, I'll do my best. Sorry I totally forgot about it.

            :::
        `,
      BikerBro: unindent`
            ### 4/1/2024

            ::: sms name:Rufus image:rufus_red

            Please call me back when you read this

            :::

            ### 10/1/2024

            ::: sms name:Rufus image:rufus_red

            We need to discuss some things regarding the shop. I know
            you want to go soon after the funeral, so how about I come with you to
            Greenfield the next day, then we'll have plenty of time to discuss in person?

            :::

            ::: sms name:Rufus image:rufus_red

            Besides I want to see your new place

            :::

            
            ::: sms name:Rebecca image:rebecca_red me

            Sounds good, but I need to go back the same day for a meeting.

            :::

            ::: sms name:Rufus image:rufus_red
            
            So you are not even staying for the reception?

            :::

            ::: sms name:Rebecca image:rebecca_red me

            I'm afraid not. I'm trying to graduate and get my startup off the
            ground. It's all really bad timing for me, I have to make some bad choices.

            :::

            ::: sms name:Rufus image:rufus_red
            
            I think you need to reconsider your priorities. Are you sure you can't
            just stay one night?

            :::

            
            ::: sms name:Rebecca image:rebecca_red me

            I'm really sorry.

            :::

            ::: sms name:Rufus image:rufus_red
            
            How can I get to your place then?

            :::

            ### 11/1/2024


            ::: sms name:Rebecca image:rebecca_red me

            I asked Judy, she can can drive you

            :::

            ::: sms name:Rufus image:rufus_red

            Seriously?

            :::
        `,
      JuStar: unindent`
            ### 11/1/2024

            ::: sms name:Rebecca image:rebecca_red me
            
            Hi, want to meet up on the 12th? And maybe also bring Rufus if you are coming anyways ;)

            :::

            ::: sms name:Judy image:judy_goldridge

            Very subtle, but sure I'd like an excuse to meet you and especially Rufus again

            :::
        `,
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

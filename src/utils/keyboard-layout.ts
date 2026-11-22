import { VirtualKeyboardLayout } from "mathlive";

export const LAYOUTS: VirtualKeyboardLayout[] = [
  {
    label: "123",
    tooltip: "Numeric",
    rows: [
      [
        {
          latex: "x",
          shift: "y",
          variants: [
            "y",
            "z",
            "t",
            "r",
            "x^2",
            "x^n",
            "x^{#?}",
            "x_n",
            "x_i",
            "x_{#?}",
            { latex: "f(#?)", class: "small" },
            { latex: "g(#?)", class: "small" }
          ]
        },
        { latex: "n", shift: "a", variants: ["i", "j", "p", "k", "a", "u"] },
        "[separator-5]",
        "[7]",
        "[8]",
        "[9]",
        "[/]",
        "[separator-5]",
        {
          latex: "\\exponentialE",
          shift: "\\ln",
          variants: [
            "\\exp",
            "\\times 10^{#?}",
            "\\ln",
            "\\log_{10}",
            "\\log",
            "\\lg",
            "\\operatorname{lb}"
          ]
        },
        {
          latex: "\\imaginaryI",
          variants: ["\\Re", "\\Im", "\\imaginaryJ", "\\Vert #0 \\Vert"]
        },
        {
          latex: "\\pi",
          shift: "\\sin",
          variants: [
            "\\prod",
            { latex: "\\theta", aside: "theta" },
            { latex: "\\rho", aside: "rho" },
            { latex: "\\tau", aside: "tau" },
            "\\sin",
            "\\cos",
            "\\tan"
          ]
        }
      ],
      [
        {
          label: "<",
          latex: "<",
          class: "hide-shift",
          shift: { latex: "\\le", label: "\u2264" }
        },
        {
          label: ">",
          latex: ">",
          class: "hide-shift",
          shift: { latex: "\\ge", label: "\u2265" }
        },
        "[separator-5]",
        "[4]",
        "[5]",
        "[6]",
        "[*]",
        "[separator-5]",
        {
          class: "hide-shift",
          latex: "#@^2}",
          shift: "#@^{\\prime}}"
        },
        {
          latex: "#@^{#0}}",
          class: "hide-shift",
          shift: "#@_{#?}"
        },
        {
          class: "hide-shift",
          latex: "\\sqrt{#0}",
          shift: { latex: "\\sqrt[#0]{#?}}" }
        }
      ],
      [
        "[(]",
        "[)]",
        "[separator-5]",
        "[1]",
        "[2]",
        "[3]",
        "[-]",
        "[separator-5]",
        {
          latex: "\\int^{\\infty}_{0}\\!#?\\,\\mathrm{d}x",
          class: "small hide-shift",
          shift: "\\int",
          variants: [
            { latex: "\\int_{#?}^{#?}", class: "small" },
            { latex: "\\int", class: "small" },
            { latex: "\\iint", class: "small" },
            { latex: "\\iiint", class: "small" },
            { latex: "\\oint", class: "small" },
            "\\mathrm{d}x",
            { latex: "\\dfrac{\\mathrm{d}}{\\mathrm{d} x}", class: "small" },
            { latex: "\\frac{\\partial}{\\partial x}", class: "small" },
            "\\partial"
          ]
        },
        {
          label: "[hide-keyboard]",
          width: 1
        },
        { label: "[backspace]", width: 1 }
      ],
      [
        { label: "[shift]", width: 2 },
        "[separator-5]",
        "[0]",
        "[.]",
        "[=]",
        "[+]",
        "[separator-5]",
        "[left]",
        "[right]",
        { label: "[action]", width: 1 }
      ]
    ]
  },
  {
    label: "abc",
    labelClass: "MLK__tex-abc",
    tooltip: "Roman Letters",
    rows: [
      [
        { label: "1", variants: "1" },
        { label: "2", variants: "2" },
        { label: "3", variants: "3" },
        { label: "4", variants: "4" },
        { label: "5", shift: { latex: "\\frac{#@}{#?}" }, variants: "5" },
        { label: "6", shift: { latex: "#@^#?" }, variants: "6" },
        { label: "7", variants: "7" },
        { label: "8", shift: { latex: "\\times" }, variants: "8" },
        { label: "9", shift: { label: "(", latex: "(" }, variants: "9" },
        { label: "0", shift: { label: ")", latex: ")" }, variants: "0" }
      ],
      [
        {
          "label": "q",
          "class": "hide-shift",
          "shift": {
            "label": "Q"
          },
          "variants": "q"
        },
        {
          "label": "w",
          "class": "hide-shift",
          "shift": {
            "label": "W"
          }
        },
        {
          "label": "e",
          "class": "hide-shift",
          "shift": {
            "label": "E",
            "variants": "E"
          },
          "variants": "e"
        },
        {
          "label": "r",
          "class": "hide-shift",
          "shift": {
            "label": "R"
          },
          "variants": "r"
        },
        {
          "label": "t",
          "class": "hide-shift",
          "shift": {
            "label": "T"
          }
        },
        {
          "label": "y",
          "class": "hide-shift",
          "shift": {
            "label": "Y",
            "variants": "Y"
          },
          "variants": "y"
        },
        {
          "label": "u",
          "class": "hide-shift",
          "shift": {
            "label": "U",
            "variants": "U"
          },
          "variants": "u"
        },
        {
          "label": "i",
          "class": "hide-shift",
          "shift": {
            "label": "I",
            "variants": "I"
          },
          "variants": "i"
        },
        {
          "label": "o",
          "class": "hide-shift",
          "shift": {
            "label": "O",
            "variants": "O"
          },
          "variants": "o"
        },
        {
          "label": "p",
          "class": "hide-shift",
          "shift": {
            "label": "P"
          },
          "variants": "p"
        }
      ],
      [
        "[separator-5]",
        {
          "label": "a",
          "class": "hide-shift",
          "shift": {
            "label": "A",
            "variants": "A"
          },
          "variants": "a"
        },
        {
          "label": "s",
          "class": "hide-shift",
          "shift": {
            "label": "S"
          }
        },
        {
          "label": "d",
          "class": "hide-shift",
          "shift": {
            "label": "D"
          },
          "variants": "d"
        },
        {
          "label": "f",
          "class": "hide-shift",
          "shift": {
            "label": "F"
          }
        },
        {
          "label": "g",
          "class": "hide-shift",
          "shift": {
            "label": "G"
          },
          "variants": "g"
        },
        {
          "label": "h",
          "class": "hide-shift",
          "shift": {
            "label": "H"
          },
          "variants": "h"
        },
        {
          "label": "j",
          "class": "hide-shift",
          "shift": {
            "label": "J"
          },
          "variants": "j"
        },
        {
          "label": "k",
          "class": "hide-shift",
          "shift": {
            "label": "K"
          }
        },
        {
          "label": "l",
          "class": "hide-shift",
          "shift": {
            "label": "L"
          },
          "variants": "l"
        },
        "[separator-5]"
      ],
      [
        "[shift]",
        {
          "label": "z",
          "class": "hide-shift",
          "shift": {
            "label": "Z"
          },
          "variants": "z"
        },
        {
          "label": "x",
          "class": "hide-shift",
          "shift": {
            "label": "X"
          }
        },
        {
          "label": "c",
          "class": "hide-shift",
          "shift": {
            "label": "C"
          },
          "variants": "c"
        },
        {
          "label": "v",
          "class": "hide-shift",
          "shift": {
            "label": "V"
          }
        },
        {
          "label": "b",
          "class": "hide-shift",
          "shift": {
            "label": "B"
          },
          "variants": "b"
        },
        {
          "label": "n",
          "class": "hide-shift",
          "shift": {
            "label": "N"
          },
          "variants": "n"
        },
        {
          "label": "m",
          "class": "hide-shift",
          "shift": {
            "label": "M"
          }
        },
        "[backspace]"
      ],
      [
        "[-]",
        "[+]",
        "[=]",
        {
          "label": " ",
          "width": 1.5
        },
        {
          "label": ",",
          "shift": ";",
          "variants": ".",
          "class": "hide-shift"
        },
        "[.]",
        "[left]",
        "[right]",
        {
          "label": "[action]",
          "width": 1.5
        }
      ]
    ],
  },
  {
    label: "&infin;\u2260\u2208",
    tooltip: "Symbols",
    rows: [
      [
        {
          latex: "\\sin",
          shift: "\\sin^{-1}",
          variants: [
            { class: "small", latex: "\\sinh" },
            { class: "small", latex: "\\sin^{-1}" },
            { class: "small", latex: "\\operatorname{arsinh}" }
          ]
        },
        "\\ln",
        {
          latex: "\\mathrm{abs}",
          insert: "\\mathrm{abs}\\left(#0\\right)"
        },
        {
          latex: "\\rightarrow",
          shift: "\\Rightarrow",
          variants: [
            { latex: "\\implies", aside: "implies" },
            { latex: "\\to", aside: "to" },
            "\\dashv",
            { latex: "\\roundimplies", aside: "round implies" }
          ]
        },
        {
          latex: "\\exists",
          variants: ["\\nexists"],
          shift: "\\nexists"
        },
        { latex: "\\in", shift: "\\notin", variants: ["\\notin", "\\owns"] },
        "\\cup",
        {
          latex: "\\overrightarrow{#@}",
          shift: "\\overleftarrow{#@}",
          variants: [
            "\\overleftarrow{#@}",
            "\\bar{#@}",
            "\\vec{#@}",
            "\\hat{#@}",
            "\\check{#@}",
            "\\dot{#@}",
            "\\ddot{#@}",
            "\\mathring{#@}",
            "\\breve{#@}",
            "\\acute{#@}",
            "\\tilde{#@}",
            "\\grave{#@}"
          ]
        },
        {
          class: "small hide-shift",
          latex: "\\lim_{#?}",
          shift: "\\lim_{x\\to\\infty}",
          variants: [
            { class: "small", latex: "\\liminf_{#?}" },
            { class: "small", latex: "\\limsup_{#?}" }
          ]
        },
        "\\exponentialE"
      ],
      [
        {
          latex: "\\cos",
          shift: "\\cos^{-1}",
          variants: [
            { class: "small", latex: "\\cosh" },
            { class: "small", latex: "\\cos^{-1}" },
            { class: "small", latex: "\\operatorname{arcosh}" }
          ]
        },
        {
          latex: "\\log",
          shift: "\\log_{10}",
          variants: ["\\log_{#0}", "\\log_{10}"]
        },
        "\\left\\vert#0\\right\\vert",
        {
          latex: "\\larr",
          shift: "\\lArr",
          variants: [
            { latex: "\\impliedby", aside: "implied by" },
            { latex: "\\gets", aside: "gets" },
            "\\lArr",
            "\\vdash",
            { latex: "\\models", aside: "models" }
          ]
        },
        {
          latex: "\\forall",
          shift: "\\lnot",
          variants: [
            { latex: "\\land", aside: "and" },
            { latex: "\\lor", aside: "or" },
            { latex: "\\oplus", aside: "xor" },
            { latex: "\\lnot", aside: "not" },
            { latex: "\\downarrow", aside: "nor" },
            { latex: "\\uparrow", aside: "nand" },
            { latex: "\\curlywedge", aside: "nor" },
            { latex: "\\bar\\curlywedge", aside: "nand" }
            // {latex:'\\barwedge', aside:'bar wedge'},
            // {latex:'\\curlyvee', aside:'curly vee'},
            // {latex:'\\veebar', aside:'vee bar'},
          ]
        },
        { latex: "\\ni", shift: "\\not\\owns" },
        "\\cap",
        {
          latex: "\\overline{#@}",
          shift: "\\underline{#@}",
          variants: [
            "\\overbrace{#@}",
            "\\overlinesegment{#@}",
            "\\overleftrightarrow{#@}",
            "\\overrightarrow{#@}",
            "\\overleftarrow{#@}",
            "\\overgroup{#@}",
            "\\underbrace{#@}",
            "\\underlinesegment{#@}",
            "\\underleftrightarrow{#@}",
            "\\underrightarrow{#@}",
            "\\underleftarrow{#@}",
            "\\undergroup{#@}"
          ]
        },
        {
          class: "hide-shift small",
          latex: "\\int",
          shift: "\\iint",
          variants: [
            { latex: "\\int_{#?}^{#?}", class: "small" },
            { latex: "\\int", class: "small" },
            { latex: "\\smallint", class: "small" },
            { latex: "\\iint", class: "small" },
            { latex: "\\iiint", class: "small" },
            { latex: "\\oint", class: "small" },
            "\\intop",
            "\\iiint",
            "\\oiint",
            "\\oiiint",
            "\\intclockwise",
            "\\varointclockwise",
            "\\ointctrclockwise",
            "\\intctrclockwise"
          ]
        },
        { latex: "\\pi", shift: "\\tau", variants: ["\\tau"] }
      ],
      [
        {
          latex: "\\tan",
          shift: "\\tan^{-1}",
          variants: [
            { class: "small", latex: "\\tanh" },
            { class: "small", latex: "\\tan^{-1}" },
            { class: "small", latex: "\\operatorname{artanh}" },
            { class: "small", latex: "\\arctan" },
            { class: "small", latex: "\\operatorname{arctg}" },
            { class: "small", latex: "\\operatorname{tg}" }
          ]
        },
        {
          latex: "\\exp",
          insert: "\\exp\\left(#0\\right)",
          variants: ["\\exponentialE^{#0}"]
        },
        "\\left\\Vert#0\\right\\Vert",
        {
          latex: "\\lrArr",
          shift: "\\leftrightarrow",
          variants: [
            { latex: "\\iff", aside: "if and only if" },
            "\\leftrightarrow",
            "\\leftrightarrows",
            "\\Leftrightarrow",
            { latex: "^\\biconditional", aside: "biconditional" }
          ]
        },
        { latex: "\\vert", shift: "!" },
        {
          latex: "#@^{\\complement}",
          aside: "complement",
          variants: [
            { latex: "\\setminus", aside: "set minus" },
            { latex: "\\smallsetminus", aside: "small set minus" }
          ]
        },
        {
          latex: "\\subset",
          shift: "\\subseteq",
          variants: [
            "\\subset",
            "\\subseteq",
            "\\subsetneq",
            "\\varsubsetneq",
            "\\subsetneqq",
            "\\nsubset",
            "\\nsubseteq",
            "\\supset",
            "\\supseteq",
            "\\supsetneq",
            "\\supsetneqq",
            "\\nsupset",
            "\\nsupseteq"
          ]
        },
        {
          latex: "#@^{\\prime}",
          shift: "#@^{\\doubleprime}",
          variants: ["#@^{\\doubleprime}", "#@\\degree"]
        },
        {
          latex: "\\mathrm{d}",
          shift: "\\partial",
          variants: [
            "\\mathrm{d}x",
            { latex: "\\dfrac{\\mathrm{d}}{\\mathrm{d} x}", class: "small" },
            { latex: "\\frac{\\partial}{\\partial x}", class: "small" },
            "\\partial"
          ]
        },
        {
          latex: "\\infty",
          variants: ["\\aleph_0", "\\aleph_1", "\\omega", "\\mathfrak{m}"]
        }
      ],
      [
        { label: "[shift]", width: 2 },
        {
          class: "box",
          latex: ",",
          shift: ";",
          variants: [";", "?"]
        },
        {
          class: "box",
          latex: "\\colon",
          shift: "\\Colon",
          variants: [
            { latex: "\\Colon", aside: "such that", class: "box" },
            { latex: ":", aside: "ratio", class: "box" },
            { latex: "\\vdots", aside: "", class: "box" },
            { latex: "\\ddots", aside: "", class: "box" },
            { latex: "\\ldotp", aside: "low dot", class: "box" },
            { latex: "\\cdotp", aside: "center dot", class: "box" },
            { latex: "\\ldots", aside: "low ellipsis", class: "box" },
            { latex: "\\cdots", aside: "center ellipsis", class: "box" },
            { latex: "\\therefore", aside: "therefore", class: "box" },
            { latex: "\\because", aside: "because", class: "box" }
          ]
        },
        {
          class: "box",
          latex: "\\cdot",
          aside: "centered dot",
          shift: "\\ast",
          variants: [
            "\\circ",
            "\\bigcirc",
            "\\bullet",
            "\\odot",
            "\\oslash",
            "\\circledcirc",
            "\\ast",
            "\\star",
            "\\times",
            "\\doteq",
            "\\doteqdot"
          ]
        },
        "[separator]",
        "[left]",
        "[right]",
        {
          label: "[backspace]",
          width: 1,
          class: "action hide-shift"
        },
        { label: "[action]", width: 1 }
      ]
    ]
  }
]

const fs = require('fs');
const vsctm = require('./vendor/main.js');
const oniguruma = require('oniguruma');

function readFile(path) {
    return new Promise((resolve, reject) => {
        fs.readFile(path, (error, data) => error ? reject(error) : resolve(data));
    })
}

// Create a registry that can create a grammar from a scope name.
const registry = new vsctm.Registry({
    onigLib: Promise.resolve({
        createOnigScanner: (sources) => new oniguruma.OnigScanner(sources),
        createOnigString: (str) => new oniguruma.OnigString(str)
    }),
    loadGrammar: (scopeName) => {
        return readFile('./syntaxes/json/makefile.json').then(data => vsctm.parseRawGrammar(data.toString(), "makefile.json"))
    }
});

registry.loadGrammar('source.makefile').then(grammar => {
    const text = `CC=gcc
CFLAGS=-I.
DEPS = hellomake.h
OBJ = hellomake.o hellofunc.o 

%.o: %.c $(DEPS)
\t$(CC) -c -o $@ $< $(CFLAGS)

hellomake: $(OBJ)
\t$(CC) -o $@ $^ $(CFLAGS)
`.split("\n");
    let ruleStack = vsctm.INITIAL;
    for (let i = 0; i < text.length; i++) {
        const line = text[i];
        const lineTokens = grammar.tokenizeLine(line, ruleStack);
        console.log(grammar._ruleId2desc.length);
        for (let j = 0; j < lineTokens.tokens.length; j++) {
            const token = lineTokens.tokens[j];
            console.log(` - token from ${token.startIndex} to ${token.endIndex} ` +
                `(${line.substring(token.startIndex, token.endIndex)}) ` +
                `with scopes ${token.scopes.join(', ')}`
            );
        }
        ruleStack = lineTokens.ruleStack;
    }
});

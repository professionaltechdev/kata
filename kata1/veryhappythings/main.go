package main

import (
    "errors"
    "fmt"
    "io/ioutil"
    "math/rand"
    "os"
    "strings"
)

func keys (m map[string][]string) (keys []string) {
    // Is there really not a built in keys method? Or at least a better way?
    keys = make([]string, len(m))

    i := 0
    for k := range m {
        keys[i] = k
        i++
    }
    return
}

func buildMap(sentence string) (trigrams map[string][]string) {
    trigrams = make(map[string][]string)

    var remainder []string = strings.Split(sentence, " ")
    for len(remainder) > 2 {
        key := strings.Join(remainder[0:2], " ")
        trigrams[key] = append(trigrams[key], remainder[2])
        remainder = remainder[1:]
    }
    return
}

func lastTwoWords(sentence string) (words []string) {
    w := strings.Split(sentence, " ")
    if len(w) < 2 {
        return w
    }
    words = w[len(w)-2:]
    return
}

func attachToSentence(trigrams map[string][]string, inSentence string) (string, error) {
    // I'm not sure if error is the right way to control flow here
    words := lastTwoWords(inSentence)
    possible := trigrams[strings.Join(words, " ")]
    if len(possible) == 0 {
        return inSentence, errors.New("No new word to add")
    }
    continuation := possible[rand.Intn(len(possible))]
    return inSentence + " " + continuation, nil
}

func main() {
    var err error

    dat, err := ioutil.ReadFile(os.Args[1])
    trigrams := buildMap(string(dat))
    k := keys(trigrams)

    var next string
    next = k[rand.Intn(len(k))]
    for err == nil {
        next, err = attachToSentence(trigrams, next)
    }
    fmt.Println(next)
}

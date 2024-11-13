import { LanguageStemmer } from "wasm-stemmers";

const text = "Hello, beautiful world! How are you today - beautiful?";
alert(
  LanguageStemmer.cleanText(text)
    .map((w) => `"${w}"`)
    .join(", ")
);

function testStemmer() {
  let stemmer = new LanguageStemmer("en");
  alert(
    stemmer
      .stemText(text)
      .map((w) => `"${w}"`)
      .join(", ")
  );
}

testStemmer();

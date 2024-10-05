# [Free Dictionary API](https://dictionaryapi.dev/)

## Overview

The FreeDictionary API provides access to a vast database of word definitions, synonyms, and related linguistic information. By integrating the FreeDictionary API, **That Word** enriches its language learning experience by offering precise definitions and context for vocabulary exploration.

## Endpoints

- [API](https://api.dictionaryapi.dev/api/v2/entries/en/<word>)


### Search Word Definition

- **Endpoint**: `/api/freedictionary/definition`
- **Method**: GET
- **Description**: Search for the definition of a word using its spelling.
- **Parameters**:
  - `word` (required): The word for which the definition is requested.
- **Response**: The definition of the specified word, including synonyms, antonyms, and usage examples.

### Get Synonyms

- **Endpoint**: `/api/freedictionary/synonyms`
- **Method**: GET
- **Description**: Retrieve synonyms for a given word.
- **Parameters**:
  - `word` (required): The word for which synonyms are requested.
- **Response**: A list of synonyms for the specified word.

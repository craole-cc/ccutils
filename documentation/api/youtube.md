# [YouTube API](https://developers.google.com/youtube/v3)

## Overview

The YouTube API allows **That Word** to retrieve video snippets to provide real-life context for language learning. By integrating the YouTube API, users can access a wide range of video content featuring native speakers in various contexts, including news reports, interviews, and casual conversations.

## Endpoints

### Search Videos

- **Endpoint**: `/api/youtube/search`
- **Method**: GET
- **Description**: Search for videos using keywords related to the desired language learning content.
- **Parameters**:
  - `q` (required): The query term for the search.
  - `maxResults` (optional): The maximum number of results to return (default: 10).
  - `order` (optional): The order in which the search results should be sorted (e.g., relevance, date, rating).
- **Response**: A list of video items matching the search criteria, including video IDs, titles, descriptions, and thumbnails.

### Get Video Details

- **Endpoint**: `/api/youtube/video/{videoId}`
- **Method**: GET
- **Description**: Retrieve detailed information about a specific video.
- **Parameters**:
  - `videoId` (required): The unique ID of the video.
- **Response**: Detailed metadata about the video, including title, description, duration, and channel information.

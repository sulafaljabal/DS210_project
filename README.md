# DS210_project
CDS DS 210 Final Project Repository

- Graph shape follows a power law distribution
This project is based on the Reddit Hyperlink Network Dataset provided by Stanford Network Analysis Project website. The website showcases two datasets, both that have information gathered ...

# What the dataset looked like
The dataset had a lot of information that I didn't need. The last column contained information regarding the properties of the post, such as the number of characters, the number of words, etc. The only columns I needed were the source_subreddit, target_subreddit and post_id. I had originally planned to create multiple graphs showcasing the difference in vertex distributions based on post sentiment, but I unfortunately did not have enough time to do so.
# What I did
I decided my first steps had to be creating a HashMap with the names of all subreddits as the keys and a unique identification number as values. These ID numbers were numbered 0 to n, where n is the total number of subreddits present in the data. I also created a vector of tuples which contained line by line each origin to destination subreddit pair which would be used to create my adjacency list.
I created the adjacency list by going through the vector of tuples and using the unique ID from the HashMap. I allowed for duplicates in the adjacency list as I want to see the total number of connections between subreddits and different posts can originate on one subreddit and link to the same destination subreddit. As I went through each line of the subreddit-subreddit pairs vector, I changed the adjacency list each time. 
After completing the adjacency list, I created a graph by using code provided in Professor Leonidas' lecture.
I then decided to create a vector of xy coordinates to plot the vertex connections. I did this by finding the subreddit with the highest number of outgoing links and then creating a vector of tuples with the outgoing number of links as one coordinate and the number of subreddits that each had the same number of outgoing links:
$$l$$ = number of outgoing links, $$x$$ = number of subreddits that had $$x$$ outgoing links
$$\begin{bmatrix}()(l_0= 0, x_0 = ? \\ l_1 = 1, x_1 = ? \\ \vdots \\ l_n = n, x_n = ?)\end{bmatrix}$$
# Overview of modules
1- pub mod file_and_hashmap_stuff
This public modeule contains all the functions used to 
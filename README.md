# Weather

## About:

This weather app will interact with a discord bot 
down the road, I just wanted to get the logic correct before
I had to troubleshoot the whole thing at the same time.
I will retrieve weather information somehow, for the area around me.


### Initial Problems:
- I do not know how to use an API.
    -- I haven't needed to use one, but it might be useful here.
- Will the API allow me to select a location?
- Can I use the API for free?
- Will the API be correct?
- When I get the information I don't know how to deal with it.
    -- I might just output the information for now instead of returning
        it to the function caller.
- What if I need the weather for tomorrow?
    -- Will I need to make another app? function call?


### Step 1 - Research:

    I found a free weather API for weather.gov. This means I need to learn how to
use the API.
Looking at the specs for the API, I can use this API:
    - For FREE and ACCURATE information
    - For a specific location passed in to the request.
This solves three of the initial problems. I will have to do some research on how to
use the API for starters.

### Step 2 - The API:
    Looking at the specs again I can see examples that walk you through how to use the API.
They seem pretty straightforward with the examples but the location is in lat and lon
coordinates. I don't know where the coordinates are set to but I will leave it for now,
it's a trivial change later down the road.

    I ran into another issue with the start of using the API, how do I even call it. Like
I said, I've never used one; so, finding the "on" switch to this thing left me a little
confused. Luckily, I finished the book "Zero to Production" and learned of a crate called
"Reqwest". I can use this crate to create a client and call the API.
    
    Unfortunately, I am getting the 403 "Forbidden" error. I know this from my studies 
getting my CCNA certificate. This error happens when you making a request you are not
authorized to use. The API is free and for public use, so what gives? I will check the
documentation again. "Authentication" section annotates the use of a "user agent" is required.
Back to the reqwest crate to see if I can even add one to the client.
    
    The reqwest crate can actually add a user agent section to the header. But instead of just
creating the client with client::new() function, I need to use the client::builder(). This way
we can add the headers we need to the client. Passing in the url and the user agent, we get a 
reply. Now it says we are getting a 200 Ok return. 
    
    It turns out I cannot just print the response, as it is just the 200 value. I need something 
else I just don't know what quite yet. The API says it will return a JSON value, I am not gettting
that. A sweet google search later and I found out that the reqwest crate has a get function for 
the JSON value, not all heroes wear capes. Now we output the reply and it is a wall of text that
seem to be what we need and then some. Success.


### Step 3 - The JSON:

    With all the information we have we need to figure out how to parse out what we need and leave 
out the rest. Once we get what we are looking for, we can then proceed. I looked up the JSON
Value format online and it seems to be able to return different values in the struct. I don't know 
how to iterate through the value. When I call the json value (json instance is named json lowercase)
like an array "json[0]" I get "null". I try to iterate through the value like a for loop but the JSON
Value doesn't impl an iterator. I try another for loop and iterate through like an array with a higher
number value to see if the first element is just null. I get 10 elements outputting null. Back to the 
drawing board. 

    Okay, so let's get this straight. The documentation is calling it like an array. It looks like:
        ObjectName["string"][elementPosition]
So maybe I can call: json["someString"] where the someString being something I see in the wall of 
text. It works! Now can we do another element? Still works, we're onto something.

    It works but I ran into another wall. We have "json["Properties"]" and it works. But if I try
to get the next element it is not getting anything. Scratch that, we need to use another string as
it is the next section, not a numbered element. I apologize, I had a moment. It works again lol. We
are iterating to more specific information.
For instance, for us to get the forecast, it looks like: json["properties"]["periods"][0]["detailedForecast"].


### Step 4 - The struct:

    I decided to put the information into a struct to keep track of the coupled information altogether.
When needing specific inforation about the weather, I can call it by the 'instance.attribute'. It's just what
makes sense to me so far. Naming the struct "Weather" with a capital 'w'. Giving the struct an impl (method) to
build the struct and that's it. We can iterate through the different attributes as needed, we don't need the 
classic "getters" and "setters" in this case.

    I know things are looking pretty much wrapped up but at this point I want to look at the case of what to
do with the weather forecast of tomorrow. Of course, this is a bit of a hiccup I am introducing for a "wouldn't
it be nice if" use case. But if I think it could be useful it would be better to add it rather than finidhing up
the whole weather section then trying to add it. Given the currrent struct we have so far, we have no means to 
get the weather forecast for tomorrow. 
We can:
    - add another method to get tomorrow's forecast
    - add another struct
    - call the build with a parameter for what day we want 
    - we can add attributes to the struct
I think we are going to add another struct to the existing struct. But we will add only one, the detailedForecast
attribute. We will only add this one field, as it covers all of the information we need in a larger format.


### Step 5 - Wrap up:
    With everything working as expected/needed we can put the finishing touches on to wrap it up. For starters, we
need to change the lat/long coordinates. 
To change coordinates we need:
    - to google the coordinates of the area,
    - plug it into the https://api.weather.gov/points/{lat},{long}
    - look up the https://api.weather.gov/gridpoints line under properties 
    - plug that line back into the program it'll look like = https://api.weather.gov/gridpoints/{wfo}/{new lat},{new long}/forecast
The information for now will just be outputted to the console for now. I have the ability to publicly call the struct values to get
them later when I need them.
Now we are finished and ready to move onto learning how to make a discord bot.

    









# Introduction to web development with Rust

## Running a local database in the container
To run local database with negligble overhead we can provide a local container to run the database for our 
development efforts. The Postgres database is very good for this. Please refer to Postgres DockerHub page for guidance
on how to start a local database. 

## Configuring Diesel

To configure Diesel we need to provide a database url via `DATABASE_URL` environment variable 


Well I have found some general mocking methods for Rust. You could literaly provide the same struct impl's twice. What you need to do is add a feature called "mock". Then you can add attributes for conditional compilation onto your struct, such that you use one implementation in your actual code, and other implementation in your testing code.



This achieves something you actually can not even have with Mockito. Namely once you have defined a mock, you have defined one mock to be used in all your tests. In Java different tests can have different mocks.



Now this may seem like really rigid solution. I can see why you would think that. I mean one mock, which behaves the same in all tests is usually not what you want. Well then just define your struct and its impl's within the body of your unit test, and turn on conditional compilation of that. Now you have one mock for each test.



Here is actually three methods I found you can use to provide mocks in Rust. First one is mocking with enums. Basically with any struct, provide an enum which provides mockable version of your struct. This method is the worst of all three in my opinion. Here is how you would do that in code:

        struct Data {
            first: String,
            second: String
         }
    
        enum MockableData {
            Mock(Data),
            Real(Data)
        }
    
        impl MockableData {
            fn reticulate_splines(&self) -> SomethingElse {
                match self {
                    // you get where this is going....
                    // different impls for different cases, yada,yada
                    MockableData::Mock(\_) => {}
                    MockableData::Real(\_) => {}
                }
            }
        }
Okay, now that we know that that sucks, then you could use conditional impls as in :

        struct Data {
            first: String,
            second: String
        }

        // real impl
        #[cfg(not(test))]
        impl Data {
            fn reticulate_splines(&self) {
                // reticulating
            }
        }
    
        #[cfg(test)]
        mod tests {
            // mock impl
            impl Data {
                fn reticulate_splines(&self) {
                    // lying about reticulating
                    }
                }
            }
now if you have multiple unit tests, where each one needs its own impl then you can do something like this:

    #[cfg(test)]
    mod tests {
        fn reticluating_test() {
            for i in 1..10 {
                impl Data {
                    fn new() -> Self {
                        Data {
                            first: i.to_string(),
                            second: i.to_string()
                        }
                    }
                }
            }
        }
    }

and last one is to use traits or marker traits, where you simply provide addition trait impls for your mocks.



Lastly I can tell you one thing. I am a professional developer working with Java, and using Rust as hobby. Difference between Java and Rust for me was basically: In Java everything is framework in Rust everything is just code. In Java you "need" JUnit, you "need" Mockito, you "need" Spring and "need" dependency injection, in Rust you just start out writing code. You want some tests? Create a binary crate and just code bunch of functions returning Results, and then report the results back. I know this does not work in enterprise environment, but it may lead you to common patterns.



Who knows maybe exactly you will develop the next JUnit for Rust.
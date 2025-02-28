<h1>What is a class?</h1>
<p>
    A class is a blueprint for creating objects. It defines the properties and behaviors of objects. 
    The properties are the attributes of the object, and the behaviors are the methods that the object can perform.
</p>
<p>
    In other words, a class defines a type of object. A class is <span class="italic">not</span> an object.
</p>
<p>
    For example, the following is the definition of a Human according to a dictionary. However, we won't
    say that there is an actual human in the dictionary.
</p>
<enhanced:img src="$lib/assets/cs1420/oop2/human-definition.png" class="image" />
<p>
    Your professor and your TAs are <span class="italic">instances</span> of a human. The dictionary's definition
    describes how we are supposed to look and behave.
</p>
<p>
    With that in mind, how do we define a human in Java?
</p>
<pre>
public class Human &lbrace;
    private String name;
    private int age;

    public void sayHello() &lbrace;
        System.out.println("Hello, my name is " + name);
    &rbrace;
&rbrace;
</pre>
<p>
    According to Java, a Human has a name and an age, and a <span class="mono">Human</span> can say hello. Instead of the birds and the bees, we
    can make an instance of a Human with the <span class="mono">new</span> keyword.
</p>
<pre>
public class MyProgram &lbrace;
    public static void main(String[] args) &lbrace;
        Human professor = new Human();

        // This won't work because the name is private
        professor.name = "Dr. Smith";

        // This will work because the sayHello method is public
        professor.sayHello();
    &rbrace;
&rbrace;
</pre>

<p>
    What's important to note is that even though <span class="mono">name</span> is <span class="mono">private</span>, we are able to indirectly access it through the <span class="mono">sayHello</span> method.
    This is the main concept behind encapsulation. You will slowly learn why encapsulation is important as you write bigger and bigger
    programs with more and more people.
</p>

<h1>What is inheritance?</h1>
<p>
    Have you ever encountered this situation when searching up a word in the dictionary?
</p>
<enhanced:img src="$lib/assets/cs1420/oop2/lemmings-definition.png" class="half-image mt-4" />
<enhanced:img src="$lib/assets/cs1420/oop2/lemming-definition.png" class="image mt-4" />
<p>
    The definition of lemmings depends on the definition of lemming. To fully define the word lemmings,
    we also need to search for the definition of lemming. In other words, lemmings <span class="italic">inherits</span>
    lemming.
</p>
<p>
    The main benefit of this for the people who write the dictionary is that they don't have to rewrite the definition of
    lemmings if the definition of lemming changes. We achieve this in Java with the following syntax.
</p>

<pre>
public class Teacher extends Human &lbrace;
    private String subject;

    public void teach() &lbrace;
        System.out.println("I am teaching " + subject);
    &rbrace;

    // This method will not compile because `name` is private
    public void hostLecture() &lbrace;
        System.out.println("Welcome to my lecture, I am " + name);
    &rbrace;
&rbrace;
</pre>

<p>
    In every regard, a <span class="mono">Teacher</span> is a <span class="mono">Human</span>. A <span class="mono">Teacher</span> has a <span class="mono">name</span> and an <span class="mono">age</span>, and a <span class="mono">Teacher</span> can <span class="mono">sayHello</span>.
    All of the things that a <span class="mono">Human</span> can do, a <span class="mono">Teacher</span> can do as well. However, a <span class="mono">Teacher</span> can also <span class="mono">teach</span>. We say that <span class="mono">Teacher</span> is a subclass of <span class="mono">Human</span>.
    Or we can also say that <span class="mono">Human</span> is the superclass of <span class="mono">Teacher</span>.
</p>
<p>
    In Java, a class can only extend one other class. In other languages, a class can extend multiple classes. This is called multiple inheritance.
</p>
<p>
    The most immediate benefit of inheritance is that you can reuse your code. The less you write the less you need to fix, in theory. We'll see a stronger
    benefit later on.
</p>
<p>
    If we wanted <span class="mono">Teacher</span> to have access to <span class="mono">name</span>, we would have to make it <span class="mono">protected</span> instead of <span class="mono">private</span>. While <span class="mono">public</span> 
    also works, it also makes it accessible by any other class instead of just subclasses, violating the principle of encapsulation.
</p>

<pre>
public class MyProgram &lbrace;
    public static void main(String[] args) &lbrace;
        // Since a Teacher is a Human, we can do this
        Human professor = new Teacher();

        // This works
        professor.sayHello();

        // This will not work
        professor.teach();

        // The inverse is not true as well
        // This will not compile, because not all Humans are Teachers
        Teacher teacher = new Human();
    &rbrace;
&rbrace;
</pre>

<p>
    The static type of <span class="mono">professor</span> is <span class="mono">Human</span>, but the dynamic type is <span class="mono">Teacher</span>. This is why we can call <span class="mono">sayHello</span> but not <span class="mono">teach</span>.
    When Java checks if a method can be called, it only looks at the static type. This is why <span class="mono">professor.teach()</span> will not compile.
    This might sound weird, but it actually simplifies the checks that we need to do. Instead of worrying about any value that could go in the variable,
    we only need to worry about the static type.
</p>

<p>
    In the above example, a <span class="mono">Teacher</span> will <span class="mono">sayHello</span> in the exact same way as every other human.
    However, we can change that.
</p>

<pre>
public class Teacher extends Human &lbrace;
    private String subject;

    public void sayHello() &lbrace;
        System.out.println("Hello, I am a teacher. My name is " + name);
    &rbrace;

    public void teach() &lbrace;
        System.out.println("I am teaching " + subject);
    &rbrace;

    // This method will not compile because `name` is private
    public void hostLecture() &lbrace;
        System.out.println("Welcome to my lecture, I am " + name);
    &rbrace;
&rbrace;

public class MyProgram &lbrace;
    public static void main(String[] args) &lbrace;
        Human professor = new Teacher();

        // This will say
        //
        // "Hello, I am a teacher. My name is Dr. Smith"
        //
        // instead of
        //
        // "Hello, my name is Dr. Smith"
        //
        // even though the static type is Human
        professor.sayHello();
    &rbrace;
&rbrace;
</pre>

<p>
    This is called method overriding. When a subclass has a method with the same name and parameters as a superclass, the subclass's method will be called instead of the superclass's method.
    Now, consider the following example:
</p>

<pre>
public class MyProgram &lbrace;
    public static void main(String[] args) &lbrace;
        // Assume that you don't know what is in this variable
        Human h = ...;

        // What will be printed?
        // Well, there are 2 possibilities depending on if h is a Teacher or not
        // In this scenario, we say that sayHello() has 2 forms
        //
        // This is an example of polymorphism, not method overloading
        h.sayHello();
    &rbrace;
&rbrace;
</pre>

<p>
    If you forgot what method overloading is, it is when a class has multiple methods with the same name but different parameters. The method that is called depends on the parameters that are passed in.
</p>

<pre>
public class Human &lbrace;
    private String name;
    private int age;

    public void sayHello() &lbrace;
        System.out.println("Hello, my name is " + name);
    &rbrace;

    public void sayHello(int count) &lbrace;
        for (int i = 0; i &lt; count; i++) &lbrace;
            System.out.println("Hello, my name is " + name);
        &rbrace;
    &rbrace;
&rbrace;

public class MyProgram &lbrace;
    public static void main(String[] args) &lbrace;
        Human h = new Human();

        // This will call the first method
        h.sayHello();

        // This will call the second method
        h.sayHello(3);
        
        // Since the method signatures are different, we can't say
        // that this is an example of polymorphism
        // This is just method overloading
    &rbrace;
&rbrace;
</pre>

<h1>What is <span class="mono">abstract</span>?</h1>
<p>
    Consider an application that draws Rectangles and Circles. Here's what I would write if I were trying to
    avoid inheritance.
</p>

<pre>
public class Rectangle &lbrace;
    private int width;
    private int height;

    public void draw() &lbrace;
        // Draw a rectangle
    &rbrace;
&rbrace;

public class Circle &lbrace;
    private int radius;

    public void draw() &lbrace;
        // Draw a circle
    &rbrace;
&rbrace;

public class MyProgram &lbrace;
    public static void main(String[] args) &lbrace;
        Rectangle[] rects = ...;
        Circle[] circles = ...;

        for (int i = 0; i &lt; rects.length; i++) &lbrace;
            rects[i].draw();
        &rbrace;

        for (int i = 0; i &lt; circles.length; i++) &lbrace;
            circles[i].draw();
        &rbrace;
    &rbrace;
&rbrace;
</pre>

<p>
    For each shape, I need to create an array and a for loop. What if I want to add a new shape? I would have to create a new class, make another array,
    and add another for loop. This is not scalable. Let's try to use inheritance.
</p>

<pre>
public class Shape &lbrace;
&rbrace;

public class Rectangle extends Shape &lbrace;
    private int width;
    private int height;

    public void draw() &lbrace;
        // Draw a rectangle
    &rbrace;
&rbrace;

public class Circle extends Shape &lbrace;
    private int radius;

    public void draw() &lbrace;
        // Draw a circle
    &rbrace;
&rbrace;

public class MyProgram &lbrace;
    public static void main(String[] args) &lbrace;
        Shape[] shapes = ...;

        for (int i = 0; i &lt; shapes.length; i++) &lbrace;
            // This will not compile
            // Remember, Java only looks at the static type, which is Shape
            // Now check, does Shape have a draw method?
            shapes[i].draw();
        &rbrace;
    &rbrace;
&rbrace;
</pre>

<p>
    How can we fix this? We can add a <span class="mono">draw</span> method to <span class="mono">Shape</span>, but what exactly would that method do?
    Should it do nothing? Should it throw an exception? What does it even mean to make an instance of a <span class="mono">Shape</span>? The optimal solution
    would be to just leave the method unimplemented, but will Java allow us to do that? Yes, with the <span class="mono">abstract</span> keyword.
</p>

<pre>
// In lecture, Shape was an interface.
// We'll learn about that later on in this page
public abstract class Shape &lbrace;
    public abstract void draw();
&rbrace;

public class Rectangle extends Shape &lbrace;
    private int width;
    private int height;

    // If we did not implement this method, the program would not compile
    public void draw() &lbrace;
        // Draw a rectangle
    &rbrace;
&rbrace;

public class Circle extends Shape &lbrace;
    private int radius;

    public void draw() &lbrace;
        // Draw a circle
    &rbrace;
&rbrace;
</pre>

<p>
    An <span class="mono">abstract</span> class is a class that cannot be instantiated. It is meant to be a blueprint for other classes. If a class has an <span class="mono">abstract</span> method, then the class must also be <span class="mono">abstract</span>.
    An <span class="mono">abstract</span> class can have any number of <span class="mono">abstract</span> methods.
</p>

<pre>
public class MyProgram &lbrace;
    public static void main(String[] args) &lbrace;
        Shape[] shapes = ...;

        for (int i = 0; i &lt; shapes.length; i++) &lbrace;
            // This works now because there are only two types of shapes
            // that can be in the array: Rectangle and Circle
            //
            // Both classes actually implemented the draw method,
            // so this line will always call a valid draw method
            shapes[i].draw();
        &rbrace;
    &rbrace;
&rbrace;
</pre>

<p>
    No matter how many new shapes we want to add, as long as those classes extend <span class="mono">Shape</span> and implement the <span class="mono">draw</span> method, we can add them to the array and the for loop will still work.
    Abstract classes allow us to make more complex hierarchies of classes.
</p>

<h1>What is an interface?</h1>

<p>
    Lets imagine that we are the creators of Java. One of the most common actions that programmers do is sort things. Why don't we implement some
    sorting algorithms for them?
</p>

<pre>
public class Sorter &lbrace;
    public static sortIntegers(int[] arr) &lbrace;
        // Sort the array
    &rbrace;

    public static sortStrings(String[] arr) &lbrace;
        // Sort the array
    &rbrace;

    public static sortDoubles(double[] arr) &lbrace;
        // Sort the array
    &rbrace;

    ...
&rbrace;
</pre>

<p>
    For every type that can be sorted, we can implement a sort method. Obviously, this is not scalable. Even worse, what if some students in a random
    CS 1420 class want to sort their <span class="mono">Rectangle</span> objects by area? We can't expect them to call the creators of Java to implement
    a <span class="mono">sortRectangles</span> method.
</p>

<p>
    The secret is regardless of the type of thing being sorted, the sorting algorithm is the same. As long as we compare two things, we can sort them. If
    that's the case, why don't we sort <span class="mono">abstract</span> objects?
</p>

<pre>
public abstract class Comparable &lbrace;
    // If this object is less than other, return a negative number
    // If this object is equal to other, return 0
    // If this object is greater than other, return a positive number
    public abstract int compareTo(Comparable other);
&rbrace;

public class Sorter &lbrace;
    public static sort(Comparable[] arr) &lbrace;
        // Sort the array using the compareTo method
    &rbrace;
&rbrace;
</pre>

<p>
    Now, we can sort any type of object as long as it extends <span class="mono">Comparable</span> and implements the <span class="mono">compareTo</span> method.
    Unfortunately, we have a big problem. If we want to sort <span class="mono">Rectangle</span> objects by area we would have to make <span class="mono">Rectangle</span> extend <span class="mono">Comparable</span>,
    but <span class="mono">Rectangle</span> already extends <span class="mono">Shape</span>. Java does not allow multiple inheritance because it causes a lot of problems.
</p>

<p>
    To solve this, we can make an interface instead. An interface is like an <span class="mono">abstract</span> class, but it can only have <span class="mono">abstract</span> methods. An interface can be implemented by any class.
</p>

<pre>
// There is already a Comparable interface in Java
// but it looks slightly different as it uses generics.
// Something you will learn about soon
public interface Comparable &lbrace;
    // If this object is less than other, return a negative number
    // If this object is equal to other, return 0
    // If this object is greater than other, return a positive number
    //
    // We don't need to specify that this method is abstract because all methods in an interface are abstract
    public int compareTo(Comparable other);
&rbrace;

// This is unchanged, but still works
public class Sorter &lbrace;
    public static sort(Comparable[] arr) &lbrace;
        // Sort the array using the compareTo method
    &rbrace;
&rbrace;
</pre>

<p>
    Now, we can make <span class="mono">Rectangle</span> implement <span class="mono">Comparable</span> and <span class="mono">compareTo</span> to sort by area. A class
    can only extend one other class (abstract or not), but it can implement multiple interfaces.
</p>

<pre>
public class Rectangle extends Shape implements Comparable &lbrace;
    private int width;
    private int height;

    // If this object is less than other, return a negative number
    // If this object is equal to other, return 0
    // If this object is greater than other, return a positive number
    public int compareTo(Comparable other) &lbrace;
        Rectangle r = (Rectangle) other;
        return width * height - r.width * r.height;
    &rbrace;

    public void draw() &lbrace;
        // Draw a rectangle
    &rbrace;
&rbrace;

// If we wanted to implement multiple interfaces, we just separate them with a comma
public class Circle extends Shape implements Comparable, Serializable, MyCoolInterface &lbrace;
    ...
&rbrace;
</pre>

<p>
    If that's the case, what's the difference between an interface and an <span class="mono">abstract</span> class? The main difference is that an <span class="mono">abstract</span> class can have
    regular methods and fields, but an interface can only have <span class="mono">abstract</span> methods. Use whichever one is required in the
    assignments.
</p>

<pre>
public abstract class MyAbstractClass &lbrace;
    public int value;

    public void doSomething() &lbrace;
        // Do something
    &rbrace;

    // An abstract class is not even required to have abstract methods
&rbrace;

public interface MyInterface &lbrace;
    // An interface is also not required to have abstract methods,
    // but it can't have regular methods
&rbrace;
</pre>


<h1>The object hierarchy</h1>
<p>
    It is possible to describe the hierarchy of objects in Java with a tree. At the top of the tree is <span class="mono">Object</span>. Every class in Java
    is a subclass of <span class="mono">Object</span> even if you don't write <span class="mono">extends</span>. This is why every object in Java has a
    <span class="mono">toString</span> method and an <span class="mono">equals</span> method.
</p>
<p>
    Interfaces do not extend <span class="mono">Object</span>, but they can extend other interfaces. However, you probably won't encounter that.
</p>
<enhanced:img src="$lib/assets/cs1420/oop2/obj-hierarchy.png" class="image mt-4" />
<p>
    You will most likely receive questions about the object hierarchy in the exam. Make sure to understand the relationships between the classes.
</p>

<div style:margin-top="8rem">
</div>

<style>
    h1  {
        font-size: 2rem;
        font-weight: bolder;
        margin-top: 1rem;    
    }
    p, pre {
        margin-top: 1rem;
    }
    .image {
        width: min(45rem, 100%);
        height: auto;
    }
    .half-image {
        width: min(22.5rem, 100%);
        height: auto;
    }
    pre {
        background-color: #222222;
        padding: 0.5rem;
        border-radius: 0.5rem;
    }
    .mono {
        font-family: monospace;
    }
</style>
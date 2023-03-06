// refs can only exist when the component is rendered and attached to the dom.
//this can only be done currently by returning it in the view.
trait NodeRef {
    type Ref;

    // as tuple so the user always uses it.
    //lint rule would help here
    fn node_ref() -> (Self::Ref, Self);
}

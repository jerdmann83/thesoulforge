defmodule Kvstore.BucketTest do
  use ExUnit.Case, async: true

  setup do
    bucket = start_supervised!(Kvstore.Bucket)
    %{bucket: bucket}
  end


  test "stores values by key", %{bucket: bucket} do
    assert Kvstore.Bucket.get(bucket, "milk") == nil

    Kvstore.Bucket.put(bucket, "milk", 3)
    assert Kvstore.Bucket.get(bucket, "milk") == 3
  end

  test "pops values by key", %{bucket: bucket} do
    assert Kvstore.Bucket.get(bucket, "milk") == nil

    Kvstore.Bucket.put(bucket, "milk", 3)
    assert Kvstore.Bucket.get(bucket, "milk") == 3

    assert Kvstore.Bucket.pop(bucket, "milk") == 3
    assert Kvstore.Bucket.pop(bucket, "milk") == nil

    assert Kvstore.Bucket.pop(bucket, "nope") == nil
  end

  test "are temporary workers" do
    assert Supervisor.child_spec(Kvstore.Bucket, []).restart == :temporary
  end

  test "something racy here", %{bucket: bucket} do
    ## Compute in the agent/server
    #def get_something(agent) do
      #Agent.get(agent, fn state -> do_something_expensive(state) end)
    #end

    ## Compute in the agent/client
    #def get_something(agent) do
      #Agent.get(agent, &(&1)) |> do_something_expensive()
    #end
  end
end


defmodule Kvstore.RegistryTest do
  use ExUnit.Case, async: true

  setup do
    registry = start_supervised!(Kvstore.Registry)
    %{registry: registry}
  end

  test "spawns buckets", %{registry: registry} do
    assert Kvstore.Registry.lookup(registry, "shopping") == :error

    Kvstore.Registry.create(registry, "shopping")
    assert {:ok, bucket} = Kvstore.Registry.lookup(registry, "shopping")

    Kvstore.Bucket.put(bucket, "milk", 1)
    assert Kvstore.Bucket.get(bucket, "milk") == 1
    assert Kvstore.Bucket.get(bucket, "nope") == nil
  end

  test "removes buckets on exit", %{registry: registry} do
    Kvstore.Registry.create(registry, "shopping")
    {:ok, bucket} = Kvstore.Registry.lookup(registry, "shopping")
    Agent.stop(bucket)
    assert Kvstore.Registry.lookup(registry, "shopping") == :error
  end

  test "handles arbitrary messages", %{registry: registry} do
    {:ok, pid} = Kvstore.Registry.start_link([])
    send(pid, {:ok, "deal with it"})
    assert true
  end

  test "removes bucket on crash", %{registry: registry} do
    Kvstore.Registry.create(registry, "shopping")
    {:ok, bucket} = Kvstore.Registry.lookup(registry, "shopping")

    Agent.stop(bucket, :shutdown)
    assert Kvstore.Registry.lookup(registry, "shopping") == :error
  end

end

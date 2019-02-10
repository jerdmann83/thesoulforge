defmodule Kvstore do
  use Application

  def start(_type, _args) do
    Kvstore.Supervisor.start_link(name: Kvstore.Supervisor)
  end

  @moduledoc """
  Documentation for Kvstore.
  """

  @doc """
  Hello world.

  ## Examples

      iex> Kvstore.hello()
      :world

  """
  def hello do
    :world
  end
end
